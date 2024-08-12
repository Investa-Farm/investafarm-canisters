use std::str::FromStr;

use b3_utils::api::{InterCall, CallCycles};
use evm_rpc_canister_types::{
    EthSepoliaService, GetTransactionReceiptResult, MultiGetTransactionReceiptResult, RpcServices
}; 
use candid::Nat;
use b3_utils::ledger::{ICRCAccount, ICRC1, ICRC1TransferArgs, ICRC1TransferResult};
use b3_utils::caller_is_controller;

use crate::ck_eth::minter;
use crate::receipt; 
use crate::payments; 
use crate::transaction_fees;

use num_traits::cast::ToPrimitive;

use crate::ck_eth_payments::EVM_RPC; 

const USDC_HELPER: &str = "0x70e02abf44e62da8206130cd7ca5279a8f6d6241"; // Helper contract address for ckSepoliaUSDC 
const USDC_LEDGER: &str = "apia6-jaaaa-aaaar-qabma-cai"; 
const USDC_MINTER: &str = "jzenf-aiaaa-aaaar-qaa7q-cai";

// Testing get receipt function 
#[ic_cdk::update]
async fn get_usdc_receipt(hash: String) -> String {
    let receipt = eth_get_transaction_receipt(hash).await.unwrap();
    let wrapper = receipt::ReceiptWrapper::from(receipt);
    serde_json::to_string(&wrapper).unwrap()
}

// Verifying ckUSDC transaction 
#[ic_cdk::update]
async fn verify_usdc_transaction(hash: String, farm_id: u64, investor_id: u64) -> Result<receipt::VerifiedTransactionDetails, String> {
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
    if receipt_data.to != USDC_HELPER {
        return Err("Minter address does not match".to_string());
    }

    // Verify the principal in the logs matches the deposit principal
    let log_principal = receipt_data.logs.iter()
        .find(|log| log.topics.get(3).map(|topic| topic.as_str()) == Some(&crate::ck_eth_payments::canister_deposit_principal()))
        .ok_or_else(|| "Principal does not match or missing in logs".to_string())?;

    // Extract relevant transaction details
    let amount =  log_principal.data.clone();
    let from_address = receipt_data.from.clone();

    // Convert that amount to f64 
    let amount_f64 = hex_string_with_0x_to_f64(amount.clone());

    // Deduct 0.5% from the amount (transactional fees) 
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

// Helper function to convert a hex string with 0x prefix to f64
fn hex_string_with_0x_to_f64(hex_string: String) -> f64 {
    let hex_string = hex_string.trim_start_matches("0x");
    let bytes = hex::decode(hex_string).expect("Failed to decode hex string");
    let big_uint = num_bigint::BigUint::from_bytes_be(&bytes);
    let nat = Nat::from(big_uint);
    nat.0.to_f64().unwrap_or(f64::MAX)
}

// ---> ckUSDC FUNCTIONALITIES <--- //  

#[ic_cdk::update]
async fn ckusdc_balance() -> Nat {
    let account = ICRCAccount::new(ic_cdk::id(), None);

    ICRC1::from(USDC_LEDGER).balance_of(account).await.unwrap()
}

// Transfering a specified amount of ckUSDC to another account 
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

// Withdrawing ckETH from the canister
#[ic_cdk::update(guard = "caller_is_controller")]
async fn ckusdc_withdraw(amount: Nat, recipient: String) -> minter::WithdrawalResult {
    let withdraw = minter::WithdrawalArg{ 
        amount, 
        recipient
    }; 
    
    InterCall::from(USDC_MINTER)
    .call(
        "withdraw_eth", 
        withdraw, 
        CallCycles::NoPay
    )
    .await
    .unwrap()
}