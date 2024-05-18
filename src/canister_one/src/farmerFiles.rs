use std::{borrow::Cow, cell::RefCell};
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{memory_manager::MemoryId, BoundedStorable, DefaultMemoryImpl, Memory, StableBTreeMap, Storable }; 
use ic_cdk::{update, query};
use serde::{Serialize, Deserialize}; 

use crate::entitymanagement::{self, Error, Success}; 

// type Memory = VirtualMemory<DefaultMemoryImpl>; 

#[derive(CandidType, Serialize, Deserialize, Default, Clone)] 
pub struct FarmerReport {
    embed_url: String,
    farmer_id: u64,
    farmer_name: String,
    file_name: String,
}

impl Storable for FarmerReport {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }     
 
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }    
}

impl BoundedStorable for FarmerReport {
    const MAX_SIZE: u32 = 1024; 
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    pub static FARM_REPORTS: RefCell<StableBTreeMap<u64, FarmerReport, entitymanagement::Memory>> = 
    RefCell::new(StableBTreeMap::init(
      entitymanagement::MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    )); 
}

#[update] 
fn uploaded_farmer_report(embedurl: String, farmer_id: u64, farmer_name: String, file_name: String) -> Result<Success, Error> {
    
    let report = FarmerReport {
        embed_url: embedurl,
        farmer_id,
        farmer_name,
        file_name,
    };

    match FARM_REPORTS.with(|reports| reports.borrow_mut().insert(farmer_id, report)) {
        Some(_) => Err(Error::Error { msg: format!("A report with this farmer id already exists") } ),
        None => Ok(Success::ReportUploadedSuccesfully { msg: format!("Report uploaded successfully") } ),
    }

}

#[query]
fn get_farmer_report(farmer_id: u64) -> Option<FarmerReport> {
    FARM_REPORTS.with(|reports| reports.borrow().get(&farmer_id))
}