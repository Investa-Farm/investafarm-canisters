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

// const MINTER_ADDRESS: &str = "0xb44b5e756a894775fc32eddf3314bb1b1944dc34";
// const LEDGER: &str = "apia6-jaaaa-aaaar-qabma-cai"; // Canister responsible for keeping track of account balances and facilitating transfer of ckETH among users
// const MINTER: &str = "jzenf-aiaaa-aaaar-qaa7q-cai"; // Canister responsible for minting and burning of ckETH tokens -> When a user deposits ETH to the helper contract on Ethereum...
// // the MINTER listens for ReceivedEth events and transfers the ckETH tokens to the user's account - and similarly, when a user wants to withdraw ETH from the helper contract on Ethereum...
// // they create an ICRC-2 approval on the ledger and call the withdraw_eth on the minter

// #[derive(CandidType, Deserialize, Serialize)]
// pub struct TransferArgs {
//     amount: NumTokens,
//     to_account: Account,
// }

const RPC_URL: &str = "https://eth-sepolia.g.alchemy.com/v2/DZ4mML30eplCsoK1DGPPbhX5YfvR7ZhL";

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

    serde_json::to_string(&receipt).unwrap()
}

// Function for getting transaction receipt the transaction hash
async fn eth_get_transaction_receipt(hash: String) -> Result<receipt::Root, String> {

    // Preparing JSON RPC Payload usnig the serde_json::json!
    let rpc = json!({
        "jsonrpc": "2.0",
        "id": 0,
        "method": "eth_getTransactionReceipt",
        "params": [hash]
    }); 
    
    // Using HttpOutcall struct from b3_utils to make an HTTP POST request to the RPC URL
    let request = HttpOutcall::new(RPC_URL)
        .post(&rpc.to_string(), Some(2048))
        .send_with_closure(|response: HttpOutcallResponse| HttpOutcallResponse {
            status: response.status,
            body: response.body,
            ..Default::default()
        });
    
    match request.await {
        Ok(response) => {
            if response.status != 200u16 {
                return Err(format!("Error: {}", response.status));
            }

            let transaction = serde_json::from_slice::<receipt::Root>(&response.body)
                .map_err(|e| format!("Error: {}", e.to_string()))?;

            Ok(transaction)
        }
        Err(m) => Err(format!("Error: {}", m)),
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
