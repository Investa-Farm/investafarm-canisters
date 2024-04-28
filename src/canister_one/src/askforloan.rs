use candid::{CandidType, Encode, Decode}; 
use ic_cdk::{query, update}; 
use ic_stable_structures::memory_manager::{MemoryManager, VirtualMemory, MemoryId};
use ic_stable_structures::{StableBTreeMap, BoundedStorable, DefaultMemoryImpl, Storable};
use serde::{Deserialize, Serialize}; 
use std::cell::RefCell;
use std::time::Duration; 
use std::borrow::Cow; 
// use ic_cdk_timers::set_timer; 
use crate::entitymanagement::{self, Farmer}; 

type Memory = VirtualMemory<DefaultMemoryImpl>; 

// Loan struct 
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Loan {
  farm_id: u64, 
  farm_name: String, 
  loan_amount: u64, 
  maturity_date: Duration, 
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct NewLoan {
  farm_id: u64, 
  farm_name: String, 
  loan_amount: u64
} 


// Serializing and Deserializing Loan Data 
impl Storable for Loan {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Loan {
    const MAX_SIZE: u32 = 1024; 
    const IS_FIXED_SIZE: bool = false;
}

// Thread local for achieving interior mutability 
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    ); 

    pub static LOAN_STORAGE: RefCell<StableBTreeMap<u64, Loan, Memory>> = 
    RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
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

// Function for applying loan 
#[update]
fn apply_for_loan(new_loan: NewLoan) -> Result<Success, Error> {
// fn apply_for_loan(new_loan: NewLoan) -> Loan {
    if new_loan.farm_id == 0 || new_loan.farm_name.is_empty() || new_loan.loan_amount == 0 {
        return Err(Error::FieldEmpty { msg: format!("Kindly fill in all fields!") });
    } 

    // Get the list of farmers
    // let farmers = entitymanagement::return_farmers();

    // let mut farm_exists = false; 
    // let mut farm_id = 0; 

    // for farmer in farmers.iter() {
    //     if farmer.id == new_loan.farm_id && farmer.farm_name == new_loan.farm_name {
    //         farm_id = farmer.id; 
    //         farm_exists = true; 
    //         break; 
    //     }
    // }

    // if !farm_exists {
    //     return Err(Error::ErrorApplyingForLoan { 
    //         msg: format!("Farm ID and Farm Name don't match!") 
    //     }); 
    // }

    // Create a loan struct 
    let loan = Loan {
        farm_id: new_loan.farm_id, 
        farm_name: new_loan.farm_name, 
        loan_amount: new_loan.loan_amount, 
        maturity_date: Duration::from_secs(180 * 24 * 60 * 60), 
    };  

    let loan_clone = loan.clone();

    // Adding new loan to storage 
    LOAN_STORAGE.with(|loans| loans.borrow_mut().insert(new_loan.farm_id, loan)); 

    Ok(Success::LoanAppliedSuccesfully { 
        msg: format!("Loan applied succesfully for farm {} with ID {}", loan_clone.farm_name, loan_clone.farm_id), 
    })

} 
