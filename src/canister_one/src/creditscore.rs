use ic_cdk::update; 
use candid::Nat;
use crate::entitymanagement::{self};

use crate::ifarm_tokens;

#[update] 
async fn add_credit_score(farm_id: u64, credit_score: u64) -> Result<entitymanagement::Success, entitymanagement::Error> {

    let mut farmers = entitymanagement::return_farmers(); 

    // Transfer ifarm token to the caller
    let caller = ic_cdk::caller(); 
    let amount = Nat::from(credit_score); 
    let _ = ifarm_tokens::ifarm_transfer(caller, amount).await; 


    if let Some(farm) = farmers.iter_mut().find(|f| f.id == farm_id) {
        farm.credit_score = Some(credit_score); 

        let farm_clone = farm.clone(); 
        entitymanagement::FARMER_STORAGE.with(|service| service.borrow_mut().insert(farm_id, farm_clone)); 

        Ok(entitymanagement::Success::CreditScoreAdded { 
            msg: format!("Credit score updated for farm_id: {}", farm_id)
        })
    } else {
        Err(entitymanagement::Error::Error { msg: format!("An error occured!") })
    }
    
}