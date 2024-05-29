use ic_cdk::update; 
use crate::entitymanagement::{self};

#[update] 
fn ask_for_loan(farm_id: u64, loan_amount: u64) -> Result<entitymanagement::Success, entitymanagement::Error> { 
   let mut farmers = entitymanagement::return_farmers(); 

   if let Some(farm) = farmers.iter_mut().find(|f| f.id == farm_id) { 
        if let Some(credit_score) = farm.credit_score {
            if loan_amount > credit_score {
                return Err(entitymanagement::Error::Error { msg: format!("Loan ask should not be greater than credit score!") });
            }
        } else {
            return Err(entitymanagement::Error::Error { msg: format!("Credit score is not available!") });
        }

       farm.current_loan_ask = Some(loan_amount); 
       let farm_clone = farm.clone();  
       entitymanagement::FARMER_STORAGE.with(|service| service.borrow_mut().insert(farm_id, farm_clone)); 

       Ok(entitymanagement::Success::AppliedForLoanSuccesfully { 
        msg: format!("Loan applied succesfully for farm_id: {}", farm_id)
       })
   } else {
      Err(entitymanagement::Error::Error { msg: format!("An error occured!") })
   }
}