use std::str::FromStr;
use candid::Principal;
use b3_utils::{vec_to_hex_string_with_0x, Subaccount, caller_is_controller};
use b3_utils::api::{InterCall, CallCycles};
use evm_rpc_canister_types::{EvmRpcCanister, GetTransactionReceiptResult};
use candid::Nat;
use b3_utils::ledger::{ICRCAccount, ICRC1, ICRC1TransferArgs, ICRC1TransferResult};
use crate::ck_eth::receipt;
use crate::ck_eth::minter;
use crate::payments;
use crate::transaction_fees;
use crate::common::{eth_get_transaction_receipt, hex_string_with_0x_to_f64};

const MINTER_ADDRESS: &str = "0xb44b5e756a894775fc32eddf3314bb1b1944dc34";
const LEDGER: &str = "apia6-jaaaa-aaaar-qabma-cai";
const MINTER: &str = "jzenf-aiaaa-aaaar-qaa7q-cai";

pub const EVM_RPC_CANISTER_ID: Principal =
    Principal::from_slice(b"\x00\x00\x00\x00\x02\x30\x00\xCC\x01\x01");
pub const EVM_RPC: EvmRpcCanister = EvmRpcCanister(EVM_RPC_CANISTER_ID);

impl From<GetTransactionReceiptResult> for receipt::ReceiptWrapper {
    fn from(result: GetTransactionReceiptResult) -> Self {
        match result {
            GetTransactionReceiptResult::Ok(receipt) => {
                if let Some(receipt) = receipt {
                    receipt::ReceiptWrapper::Ok(receipt::TransactionReceiptData {
                        to: receipt.to,
                        status: receipt.status.to_string(),
                        transaction_hash: receipt.transactionHash,
                        block_number: receipt.blockNumber.to_string(),
                        from: receipt.from,
                        logs: receipt.logs.into_iter().map(|log| receipt::LogEntry {
                            address: log.address,
                            topics: log.topics,
                        }).collect(),
                    })
                } else {
                    receipt::ReceiptWrapper::Err("Receipt is None".to_string())
                }
            },
            GetTransactionReceiptResult::Err(e) => receipt::ReceiptWrapper::Err(format!("Error on Get transaction receipt result: {:?}", e)),
        }
    }
}

#[ic_cdk::query]
fn deposit_principal(principal: String) -> String {
    let principal = Principal::from_text(principal).unwrap();
    let subaccount = Subaccount::from_principal(principal);
    let bytes32 = subaccount.to_bytes32().unwrap();
    vec_to_hex_string_with_0x(bytes32)
}

#[ic_cdk::update]
async fn get_receipt(hash: String) -> String {
    let receipt = eth_get_transaction_receipt(&EVM_RPC, hash).await.unwrap();
    let wrapper = receipt::ReceiptWrapper::from(receipt);
    serde_json::to_string(&wrapper).unwrap()
}

#[ic_cdk::update]
async fn verify_cketh_transaction(hash: String, farm_id: u64, investor_id: u64) -> Result<receipt::VerifiedTransactionDetails, String> {
    let receipt = match eth_get_transaction_receipt(&EVM_RPC, hash.clone()).await {
        Ok(receipt) => receipt,
        Err(e) => return Err(format!("Failed to get receipt: {}", e)),
    };

    let receipt_data = match receipt {
        GetTransactionReceiptResult::Ok(Some(data)) => data,
        GetTransactionReceiptResult::Ok(None) => return Err("Receipt is None".to_string()),
        GetTransactionReceiptResult::Err(e) => return Err(format!("Error on Get transaction receipt result: {:?}", e)),
    };

    let success_status = Nat::from(1u8);
    if receipt_data.status != success_status {
        return Err("Transaction failed".to_string());
    }

    if receipt_data.to != MINTER_ADDRESS {
        return Err("Minter address does not match".to_string());
    }

    let log_principal = receipt_data.logs.iter()
        .find(|log| log.topics.get(2).map(|topic| topic.as_str()) == Some(&canister_deposit_principal()))
        .ok_or_else(|| "Principal does not match or missing in logs".to_string())?;

    let amount = log_principal.data.clone();
    let from_address = receipt_data.from.clone();

    let amount_f64 = hex_string_with_0x_to_f64(amount.clone());

    let deduction = amount_f64 * 0.005;
    let _ = transaction_fees::store_transaction_fee(hash.clone(), deduction);
    let new_amount = amount_f64 - deduction;

    let _ = payments::store_investments(
        farm_id,
        new_amount,
        investor_id,
        hash.clone(),
        "ckETH".to_string()
    );

    Ok(receipt::VerifiedTransactionDetails {
        amount,
        from: from_address,
    })
}

#[ic_cdk::query]
pub fn canister_deposit_principal() -> String {
    let subaccount = Subaccount::from(ic_cdk::id());
    let bytes32 = subaccount.to_bytes32().unwrap();
    vec_to_hex_string_with_0x(bytes32)
}

#[ic_cdk::update]
async fn cketh_balance() -> Nat {
    let account = ICRCAccount::new(ic_cdk::id(), None);
    ICRC1::from(LEDGER).balance_of(account).await.unwrap()
}

#[ic_cdk::update]
async fn cketh_transfer(to: String, amount: Nat) -> ICRC1TransferResult {
    let to = ICRCAccount::from_str(&to).unwrap();
    let transfer_args = ICRC1TransferArgs {
        to,
        amount,
        from_subaccount: None,
        fee: None,
        memo: None,
        created_at_time: None,
    };
    ICRC1::from(LEDGER).transfer(transfer_args).await.unwrap()
}

#[ic_cdk::update(guard = "caller_is_controller")]
async fn cketh_withdraw(amount: Nat, recipient: String) -> minter::WithdrawalResult {
    let withdraw = minter::WithdrawalArg {
        amount,
        recipient,
    };
    InterCall::from(MINTER)
        .call("withdraw_eth", withdraw, CallCycles::NoPay)
        .await
        .unwrap()
}