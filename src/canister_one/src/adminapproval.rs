use ic_cdk::update; 
use candid::CandidType; 
use serde::{Serialize, Deserialize};  
use crate::entitymanagement; 

// Error Messages 
#[derive(CandidType, Deserialize, Serialize)] 
pub enum Error {
    FarmerNotFound { msg: String }, 
    PermissionDenied { msg: String }, 
    InvestorNotFound { msg: String }
}

#[update]
pub fn verify_farmer(id: u64, verified: bool) -> Result<(), Error> {
    // let caller = ic_cdk::caller(); 

    // let allowed_principals = vec![
    //    Principal::from_text("Insert principal ID here").unwrap(), 
    //    Principal::from_text("Insert second principal ID here").unwrap()
    // ]; 

    // if allowed_principals.contains(&caller) {
    //     return Err(Error::PermissionDenied { msg: format!("You are not an admin!") })
    // }

    let mut farmers = entitymanagement::return_farmers(); 
    if let Some(farmer) = farmers.iter_mut().find(|f| f.id == id ) {
        farmer.verified = verified;

        entitymanagement::FARMER_STORAGE.with(|service| service.borrow_mut().insert(id, farmer.clone())); 
        Ok(()) 
    } else {
        Err(Error::FarmerNotFound { msg: format!("Farmer with id {} doesn't exist", id) })
    }

}

#[update] 
pub fn verify_investor(id: u64, verified: bool) -> Result<(), Error> {
    // Insert functionality for checking only the specific principal IDs have registered 

    let mut investors = entitymanagement::return_investors(); 

    if let Some(investor) = investors.iter_mut().find(|i| i.id == id) {
        investor.verified = verified; 

        entitymanagement::INVESTOR_STORAGE.with(|service| service.borrow_mut().insert(id, investor.clone())); 
        Ok(())
    } else {
        Err(Error::InvestorNotFound { msg: format!("Investor with id {} doesn't exist", id) })
    }
}

// #[update]
// pub fn 