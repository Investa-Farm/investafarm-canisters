use std::str::FromStr;
use b3_utils::api::{InterCall, CallCycles};
use evm_rpc_canister_types::GetTransactionReceiptResult;
use candid::Nat;
use b3_utils::ledger::{ICRCAccount, ICRC1, ICRC1TransferArgs, ICRC1TransferResult};
use b3_utils::caller_is_controller;
use crate::ck_eth::minter;
use crate::receipt;
use crate::payments;
use crate::transaction_fees;
use crate::common::{eth_get_transaction_receipt, hex_string_with_0x_to_f64};
use crate::ck_eth_payments::EVM_RPC;
use crate::entitymanagement::{check_entity_type, EntityType};

const USDC_HELPER: &str = "0x70e02abf44e62da8206130cd7ca5279a8f6d6241";
const USDC_LEDGER: &str = "yfumr-cyaaa-aaaar-qaela-cai";
const USDC_MINTER: &str = "jzenf-aiaaa-aaaar-qaa7q-cai"; 

#[ic_cdk::update]
async fn get_usdc_receipt(hash: String) -> String {
    let receipt = eth_get_transaction_receipt(&EVM_RPC, hash).await.unwrap();
    let wrapper = receipt::ReceiptWrapper::from(receipt);
    serde_json::to_string(&wrapper).unwrap()
}

#[ic_cdk::update]
async fn verify_usdc_transaction(hash: String, farm_id: u64, investor_id: u64) -> Result<receipt::VerifiedTransactionDetails, String> {
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

    if receipt_data.to != USDC_HELPER {
        return Err("Minter address does not match".to_string());
    }

    let log_principal = receipt_data.logs.iter()
        .find(|log| log.topics.get(3).map(|topic| topic.as_str()) == Some(&crate::ck_eth_payments::canister_deposit_principal()))
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
        "ckUSDC".to_string()
    );

    Ok(receipt::VerifiedTransactionDetails {
        amount,
        from: from_address,
    })
}

/// Store transaction fee separately
async fn store_transaction_fee(hash: String, fee: f64) -> Result<(), String> {
    transaction_fees::store_transaction_fee(hash, fee).map_err(|e| format!("Failed to store transaction fee: {}", e))
}

/// Store investment details separately
#[ic_cdk::update]
async fn store_investment(farm_id: u64, amount: f64, investor_id: u64, hash: String) -> Result<(), String> {
    // Verify caller is an Investor
    match check_entity_type() {
        EntityType::Investor => {
            let deduction = amount * 0.005;
            store_transaction_fee(hash.clone(), deduction).await?;
            let new_amount = amount - deduction;

            payments::store_investments(farm_id, new_amount, investor_id, hash, "ckUSDC".to_string())
                .map_err(|e| format!("Failed to store investment: {}", e))
        },
        _ => Err("Only investors can store investments".to_string())
    }
}

#[ic_cdk::update]
async fn ckusdc_balance() -> Nat {
    let account = ICRCAccount::new(ic_cdk::id(), None);
    ICRC1::from(USDC_LEDGER).balance_of(account).await.unwrap()
}

#[ic_cdk::update]
async fn ckusdc_transfer(to: String, amount: Nat) -> ICRC1TransferResult {
    let to = ICRCAccount::from_str(&to).unwrap();
    let transfer_args = ICRC1TransferArgs {
        to,
        amount,
        from_subaccount: None,
        fee: None,
        memo: None,
        created_at_time: None,
    };
    ICRC1::from(USDC_LEDGER).transfer(transfer_args).await.unwrap()
}

#[ic_cdk::update(guard = "caller_is_controller")]
async fn ckusdc_withdraw(amount: Nat, recipient: String) -> minter::WithdrawalResult {
    let withdraw = minter::WithdrawalArg {
        amount,
        recipient,
    };
    InterCall::from(USDC_MINTER)
        .call("withdraw_eth", withdraw, CallCycles::NoPay)
        .await
        .unwrap()
}