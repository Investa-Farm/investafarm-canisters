use ic_cdk::{call, update};
use serde::{Serialize, Deserialize}; 
use candid::{CandidType, Principal}; 

// Farmer Struct 
#[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Farmer {
  id: u64, 
  principal_id: Principal, 
  farmer_name: String, 
  farm_name: String, 
  farm_description: String, 
  amount_invested: u64, 
  investors_ids: Principal, 
  verified: bool, 
  agri_business: Option<String>, 
  insured: bool
}

// Error Messages 
#[derive(CandidType, Deserialize, Serialize)] 
pub enum Error {
   AnErrorOccured { msg: String }, 
}

#[update]
pub async fn call_testing_inter_canister() -> Result<String, Error> {
    let canister_one_id = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
    let (res,): (String,) = call(canister_one_id, "testing_inter_canister", ())
      .await.map_err(|e| Error::AnErrorOccured { msg: format!("{:?}", e) })?; 
    Ok(res)
}

#[update] 
pub async fn call_get_farmers() -> Result<Vec<Farmer>, Error> {
   let canister_one_id = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
   let (res, ): (Vec<Farmer>, ) = call(canister_one_id, "display_farms", ())
      .await.map_err(|e| Error::AnErrorOccured { msg: format!("{:?}", e) })?; 
   Ok(res)
}

ic_cdk::export_candid!();  