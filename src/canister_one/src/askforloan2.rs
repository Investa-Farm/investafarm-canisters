use std::{collections::HashMap, time::Duration}; 
use std::cell::RefCell; 
use serde::{Deserialize, Serialize}; 
use candid::CandidType;
use ic_cdk::{update};

struct Loan {
   loans: HashMap<u64, Vec<(u64, Duration)>>,  // farmer_id => [(amount, duration)]
}

thread_local! {
    static LOAN_STORAGE: RefCell<Loan> = RefCell::new(Loan { loans: HashMap::new() });
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
fn apply_for_loan(farm_id: u64, amount: u64) -> Result<Success, Error>{
    
    if farm_id == 0 || amount == 0 {
        return Err(Error::FieldEmpty { msg: format!("Kindly fill in all fields!") });
    } 

    let duration = Duration::from_secs(180 * 24 * 60 * 60); 

    LOAN_STORAGE.with(|loan| {
        let mut loan = loan.borrow_mut(); 
        let entry = loan.loans.entry(farm_id).or_insert_with(Vec::new); 
        entry.push((amount, duration)); 
    }); 

    Ok(Success::LoanAppliedSuccesfully { 
        msg: format!("Loan applied succesfully for farm"), 
    })

}