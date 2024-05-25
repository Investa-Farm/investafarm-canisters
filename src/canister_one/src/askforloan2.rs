use std::time::Duration; 
use ic_stable_structures::{memory_manager::MemoryId, BoundedStorable, StableBTreeMap, Storable }; 
use std::{borrow::Cow, cell::RefCell};
use serde::{Deserialize, Serialize}; 
use candid::{CandidType, Decode, Encode};
use ic_cdk::{update, query};

use crate::entitymanagement;
// use ic_cdk::{update};

#[derive(CandidType, Serialize, Deserialize, Default, Clone)] 
pub struct Loan {
    farmer_id: u64, 
    amount: u64, 
    duration: Duration
}

impl Storable for Loan {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }     
 
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }    
}

impl BoundedStorable for Loan {
    const MAX_SIZE: u32 = 1024; 
    const IS_FIXED_SIZE: bool = false;
}

// struct Loan {
//    loans: HashMap<u64, Vec<(u64, Duration)>>,  // farmer_id => [(amount, duration)]
// }

thread_local! {
    pub static FARM_REPORTS: RefCell<StableBTreeMap<u64, Loan, entitymanagement::Memory>> = 
    RefCell::new(StableBTreeMap::init(
      entitymanagement::MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8)))
    )); 
}

// Success Message 
#[derive(CandidType, Deserialize, Serialize)]
pub enum  Success {
   LoanAppliedSuccesfully { msg: String }     
}

// Error Message 
#[derive(CandidType, Deserialize, Serialize)]
pub enum Error {
    ErrorApplyingForLoan { msg: String }, 
    FieldEmpty { msg: String }
}


// Function for applying for a loan 
#[update] 
fn apply_for_loan(farm_id: u64, amount: u64) -> Result<Success, Error> {
    if farm_id == 0 || amount == 0 {
        return Err(Error::FieldEmpty { msg: String::from("Farm ID or amount cannot be zero") });
    }

    let loan = Loan {
        farmer_id: farm_id,
        amount,
        duration: Duration::from_secs(180 * 24 * 60 * 60), 
    };

    FARM_REPORTS.with(|reports| {
        let mut reports_borrowed = reports.borrow_mut();
        reports_borrowed.insert(farm_id, loan);
    });

    Ok(Success::LoanAppliedSuccesfully { msg: String::from("Loan applied successfully") })
}

#[query]
fn get_loan_for_farm(farm_id: u64) -> Result<Option<Loan>, Error> {
    if farm_id == 0 {
        return Err(Error::FieldEmpty { msg: String::from("Farm ID cannot be zero") });
    }

    let loan = FARM_REPORTS.with(|reports| {
        let reports_borrowed = reports.borrow();
        reports_borrowed.get(&farm_id)
    });

    Ok(loan)
}