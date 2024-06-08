use candid::{CandidType, Principal}; 

use ic_cdk_macros::*; 
use ic_ledger_types::{
    AccountIdentifier, BlockIndex, Memo, Subaccount, Tokens, DEFAULT_SUBACCOUNT, 
    MAINNET_LEDGER_CANISTER_ID
}; 

use serde::{Deserialize, Serialize}; 
use ic_cdk::api::call::RejectionCode; 

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    amount: Tokens, 
    to_principal: Principal, 
    to_subaccount: Option<Subaccount>
}

// Error Messages 
#[derive(CandidType, Deserialize)] 
pub enum Error {
    CallError(RejectionCode, String),
}

impl From<(RejectionCode, String)> for Error {
   fn from(err: (RejectionCode, String)) -> Self {
      Self::CallError(err.0, err.1)
   }    
}

// Testing whether the ICP ledger canister works 
#[update]
pub async fn test_ledger() -> Result<String, Error> {
    let ledger_id = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(); 
    let req = (); 
    let (res,): (String,) = ic_cdk::call(ledger_id, "icrc1_name", (req,)).await?;
    Ok(res) 
}

#[update] 
async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> { 
    
    ic_cdk::println!(
        "Transferring {} tokens to principal {} subaccount {:?}", 
        &args.amount, 
        &args.to_principal, 
        &args.to_subaccount
    ); 

    let to_subaccount = args.to_subaccount.unwrap_or(DEFAULT_SUBACCOUNT); 
    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0), 
        amount: args.amount, 
        fee: Tokens::from_e8s(10_000), 
        from_subaccount: None, 
        to: AccountIdentifier::new(&args.to_principal, &to_subaccount), 
        created_at_time: None 
    }; 

    ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, transfer_args)
       .await
       .map_err(|e| format!("Failed to call ledger: {:?}", e))? 
       .map_err(|e| format!("Ledger transfer error {:?}", e))
    
}
