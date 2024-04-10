use ic_cdk::{call, update};
use serde::{Serialize, Deserialize}; 
use candid::{CandidType, Principal}; 

// Error Messages 
#[derive(CandidType, Deserialize, Serialize)] 
pub enum Error {
   AnErrorOccured { msg: String }, 
}

#[update]
pub async fn call_testing_inter_canister() -> Result<String, Error> {
    let canister_one_id = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
    let (res,): (String,) = call(canister_one_id, "testing_inter_canister", ()).await.map_err(|e| Error::AnErrorOccured { msg: format!("{:?}", e) })?; 
    Ok(res)
}

ic_cdk::export_candid!();  