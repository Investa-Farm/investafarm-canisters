use candid::Principal;
// use icrc_ledger_types::icrc1::account::Account;
// use icrc_ledger_types::icrc1::transfer::{BlockIndex, NumTokens, TransferArg, TransferError};
// use serde::Serialize;
// use b3_utils::ledger::{ICRC1TransferArgs, ICRC1TransferResult};
// use std::str::FromStr;
// use b3_utils::ledger::{ICRCAccount, ICRC1};
// use b3_utils::caller_is_controller;
use b3_utils::{vec_to_hex_string_with_0x, Subaccount};
use b3_utils::outcall::{HttpOutcall, HttpOutcallResponse};
use serde_json::json;
use crate::ck_eth::receipt;
use serde::Serialize;

use evm_rpc_canister_types::{
    BlockTag, EthSepoliaService, EvmRpcCanister, GetTransactionReceiptResult, MultiGetTransactionReceiptResult, RpcError, RpcServices
}; 

use b3_utils::hex_string_with_0x_to_nat;
use candid::{Nat, CandidType, Deserialize};

const MINTER_ADDRESS: &str = "0xb44b5e756a894775fc32eddf3314bb1b1944dc34"; // Minter address for ckSepoliaETH
// const LEDGER: &str = "apia6-jaaaa-aaaar-qabma-cai"; // Canister responsible for keeping track of account balances and facilitating transfer of ckETH among users
// const MINTER: &str = "jzenf-aiaaa-aaaar-qaa7q-cai"; // Canister responsible for minting and burning of ckETH tokens -> When a user deposits ETH to the helper contract on Ethereum...
// // the MINTER listens for ReceivedEth events and transfers the ckETH tokens to the user's account - and similarly, when a user wants to withdraw ETH from the helper contract on Ethereum...
// // they create an ICRC-2 approval on the ledger and call the withdraw_eth on the minter

// #[derive(CandidType, Deserialize, Serialize)]
// pub struct TransferArgs {
//     amount: NumTokens,
//     to_account: Account,
// }

#[derive(Serialize)]
struct LogEntry {
    address: String,
    topics: Vec<String>,
}

#[derive(Serialize)]
enum ReceiptWrapper {
    Ok(TransactionReceiptData),
    Err(String),
}

#[derive(Serialize)]
struct TransactionReceiptData {
    to: String,
    status: String, 
    transaction_hash: String,
    block_number: String, 
    from: String,
    logs: Vec<LogEntry>, 
}

#[derive(Serialize, CandidType, Deserialize)]
pub struct VerifiedTransactionDetails {
    amount: String,
    from: String,
}

impl From<GetTransactionReceiptResult> for ReceiptWrapper {
    fn from(result: GetTransactionReceiptResult) -> Self {
        match result {
            GetTransactionReceiptResult::Ok(receipt) => {
                if let Some(receipt) = receipt {
                    ReceiptWrapper::Ok(TransactionReceiptData {
                        to: receipt.to,
                        status: receipt.status.to_string(),
                        transaction_hash: receipt.transactionHash,
                        block_number: receipt.blockNumber.to_string(),
                        from: receipt.from,
                        logs: receipt.logs.into_iter().map(|log| LogEntry {
                            address: log.address,
                            topics: log.topics,
                        }).collect(),
                    })
                } else {
                    ReceiptWrapper::Err("Receipt is None".to_string())
                }
            },
            GetTransactionReceiptResult::Err(e) => ReceiptWrapper::Err(format!("Error on Get transaction receipt result: {:?}", e)),
        }
    }
}


pub const EVM_RPC_CANISTER_ID: Principal =
    Principal::from_slice(b"\x00\x00\x00\x00\x02\x30\x00\xCC\x01\x01"); // 7hfb6-caaaa-aaaar-qadga-cai
pub const EVM_RPC: EvmRpcCanister = EvmRpcCanister(EVM_RPC_CANISTER_ID);

// Converting the principal object into a subaccount from the principal (necessary for depositing ETH)
#[ic_cdk::query] 
fn deposit_principal(principal: String) -> String {
    let principal = Principal::from_text(principal).unwrap(); 
    let subaccount = Subaccount::from_principal(principal); 

    let bytes32 = subaccount.to_bytes32().unwrap(); 

    vec_to_hex_string_with_0x(bytes32)
}

// Testing get receipt function 
#[ic_cdk::update]
async fn get_receipt(hash: String) -> String {
    let receipt = eth_get_transaction_receipt(hash).await.unwrap();
    let wrapper = ReceiptWrapper::from(receipt);
    serde_json::to_string(&wrapper).unwrap()
}

// Function for verifying the transaction on-chain
#[ic_cdk::update]
async fn verify_transaction(hash: String, deposit_principal: String) -> Result<VerifiedTransactionDetails, String> {
    // Get the transaction receipt
    let receipt = match eth_get_transaction_receipt(hash.clone()).await {
        Ok(receipt) => receipt,
        Err(e) => return Err(format!("Failed to get receipt: {}", e)),
    };

    // Ensure the transaction was successful
    let receipt_data = match receipt {
        GetTransactionReceiptResult::Ok(Some(data)) => data,
        GetTransactionReceiptResult::Ok(None) => return Err("Receipt is None".to_string()),
        GetTransactionReceiptResult::Err(e) => return Err(format!("Error on Get transaction receipt result: {:?}", e)),
    };

    // Check if the status indicates success (Nat 1)
    let success_status = Nat::from(1u8);
    if receipt_data.status != success_status {
        return Err("Transaction failed".to_string());
    }

    // Verify the 'to' address matches the minter address
    if receipt_data.to != MINTER_ADDRESS {
        return Err("Minter address does not match".to_string());
    }

    // Verify the principal in the logs matches the deposit principal
    let log_principal = receipt_data.logs.iter()
        .find(|log| log.topics.get(2).map(|topic| topic.as_str()) == Some(&deposit_principal))
        .ok_or_else(|| "Principal does not match or missing in logs".to_string())?;

    // Extract relevant transaction details
    //  let amount = hex_string_with_0x_to_nat(log_principal.data.clone());
    let amount =  log_principal.data.clone();
    let from_address = receipt_data.from.clone();

    Ok(VerifiedTransactionDetails {
        amount,
        from: from_address,
    })
}

#[ic_cdk::query] 
fn canister_deposit_principal() -> String {
    let subaccount = Subaccount::from(ic_cdk::id()); 

    let bytes32 = subaccount.to_bytes32().unwrap(); 

    vec_to_hex_string_with_0x(bytes32)
}

// Function for getting transaction receipt the transaction hash
async fn eth_get_transaction_receipt(hash: String) -> Result<GetTransactionReceiptResult, String> {
    // Make the call to the EVM_RPC canister
    let result: Result<(MultiGetTransactionReceiptResult,), String> = EVM_RPC 
        .eth_get_transaction_receipt(
            RpcServices::EthSepolia(Some(vec![
                EthSepoliaService::PublicNode,
                EthSepoliaService::BlockPi,
                EthSepoliaService::Ankr,
            ])),
            None, 
            hash, 
            10_000_000_000
        )
        .await 
        .map_err(|e| format!("Failed to call eth_getTransactionReceipt: {:?}", e));

    match result {
        Ok((MultiGetTransactionReceiptResult::Consistent(receipt),)) => {
            Ok(receipt)
        },
        Ok((MultiGetTransactionReceiptResult::Inconsistent(error),)) => {
            Err(format!("EVM_RPC returned inconsistent results: {:?}", error))
        },
        Err(e) => Err(format!("Error calling EVM_RPC: {}", e)),
    }    
}

// #[ic_cdk::update(guard = "caller_is_controller")]
// async fn transfer(to: String, amount: Nat) -> ICRC1TransferResult {
//     let to = ICRCAccount::from_str(&to).unwrap();

//     let transfer_args = ICRC1TransferArgs {
//         to,
//         amount,
//         from_subaccount: None,
//         fee: None,
//         memo: None,
//         created_at_time: None,
//     };

//     ICRC1::from(LEDGER).transfer(transfer_args).await.unwrap()
// }
