use ic_cdk::update; 
use crate::entitymanagement::{self};
use std::time::Duration; 

#[update] 
fn ask_for_loan(farm_id: u64, loan_amount: u64) -> Result<entitymanagement::Success, entitymanagement::Error> { 
   let mut farmers = entitymanagement::return_farmers(); 

   if let Some(farm) = farmers.iter_mut().find(|f| f.id == farm_id) { 
        if let Some(credit_score) = farm.credit_score {
            if loan_amount > credit_score {
                return Err(entitymanagement::Error::Error { msg: format!("Loan ask should not be greater than credit score!") });
            } else if farm.loaned == true {
                return Err(entitymanagement::Error::Error { msg: format!("You cannot ask for a loan while processing another loan!") });
            }
        } else {
            return Err(entitymanagement::Error::Error { msg: format!("Credit score is not available!") });
        }

       farm.current_loan_ask = Some(loan_amount); 
       // Set the time_for_funding_round_to_expire field to a duration of 4 weeks
       // farm.time_for_funding_round_to_expire = Some(Duration::from_secs(4 * 7 * 24 * 60 * 60));
       farm.time_for_funding_round_to_expire = Some(Duration::from_secs(3 * 60));

       let farm_clone = farm.clone();  
       entitymanagement::FARMER_STORAGE.with(|service| service.borrow_mut().insert(farm_id, farm_clone)); 

       Ok(entitymanagement::Success::AppliedForLoanSuccesfully { 
        msg: format!("Loan applied succesfully for farm_id: {}", farm_id)
       })
       
   } else {
      Err(entitymanagement::Error::Error { msg: format!("An error occured!") })
   }
}

#[update]
fn check_loan_expiry(farm_id: u64) -> Result<(), entitymanagement::Error> {
    let mut farmers = entitymanagement::return_farmers();
    if let Some(farm) = farmers.iter_mut().find(|f| f.id == farm_id) {
        if let Some(time_for_funding_round_to_expire) = farm.time_for_funding_round_to_expire {
            let current_time = ic_cdk::api::time();
            let expiry_time = current_time + time_for_funding_round_to_expire.as_nanos() as u64;
            if current_time >= expiry_time {
                farm.loaned = true;
                farm.loan_maturity = Some(Duration::from_secs(6 * 30 * 24 * 60 * 60));
                let farm_clone = farm.clone();
                entitymanagement::FARMER_STORAGE.with(|service| service.borrow_mut().insert(farm_id, farm_clone));
            }
        }
    } else {
        return Err(entitymanagement::Error::Error {
            msg: format!("Farm not found!"),
        });
    }
    Ok(())
}