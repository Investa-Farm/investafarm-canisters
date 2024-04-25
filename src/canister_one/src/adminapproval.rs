use crate::entitymanagement;
use candid::CandidType;
use ic_cdk::update;
use serde::{Deserialize, Serialize};

// Error Messages
#[derive(CandidType, Deserialize, Serialize)]
pub enum Error {
    FarmerNotFound { msg: String },
    PermissionDenied { msg: String },
    InvestorNotFound { msg: String },
    SupplyAgriBizNotFound { msg: String },
    FarmsAgriBizNotFound { msg: String },
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
    if let Some(farmer) = farmers.iter_mut().find(|f| f.id == id) {
        farmer.verified = verified;

        entitymanagement::FARMER_STORAGE
            .with(|service| service.borrow_mut().insert(id, farmer.clone()));
        Ok(())
    } else {
        Err(Error::FarmerNotFound {
            msg: format!("Farmer with id {} doesn't exist", id),
        })
    }
}

#[update]
pub fn verify_investor(id: u64, verified: bool) -> Result<(), Error> {
    // Insert functionality for checking only the specific principal IDs

    let mut investors = entitymanagement::return_investors();

    if let Some(investor) = investors.iter_mut().find(|i| i.id == id) {
        investor.verified = verified;

        entitymanagement::INVESTOR_STORAGE
            .with(|service| service.borrow_mut().insert(id, investor.clone()));
        Ok(())
    } else {
        Err(Error::InvestorNotFound {
            msg: format!("Investor with id {} doesn't exist", id),
        })
    }
}

// Debug this as it replicates a similar supply agri business when someone verifies it
#[update]
pub fn verify_supply_agribusiness(id: u64, verified: bool) -> Result<(), Error> {
    // Insert functionality for checking only the specific principal IDs

    let mut supply_agribusinesses = entitymanagement::return_supply_agribusiness();

    if let Some(agribiz) = supply_agribusinesses.iter_mut().find(|s| s.id == id) {
        agribiz.verified = verified;

        entitymanagement::SUPPLY_AGRIBUSINESS_STORAGE
            .with(|service| service.borrow_mut().insert(id, agribiz.clone()));
        Ok(())
    } else {
        Err(Error::SupplyAgriBizNotFound {
            msg: format!("Supply agri business with id {} doesn't exist", id),
        })
    }
}

// Debug this as it replicates a similar supply agri business when someone verifies it
#[update]
pub fn verify_farms_agribusiness(id: u64, verified: bool) -> Result<(), Error> {
    // Insert functionality for checking only the specific principal IDs

    let mut farms_agribusinesses = entitymanagement::return_farms_agribusiness();

    if let Some(agribiz) = farms_agribusinesses.iter_mut().find(|s| s.id == id) {
        agribiz.verified = verified;

        entitymanagement::FARMS_AGRIBUSINESS_STORAGE
            .with(|service| service.borrow_mut().insert(id, agribiz.clone()));
        Ok(())
    } else {
        Err(Error::FarmsAgriBizNotFound {
            msg: format!("Farmer agri business with id {} doesn't exist", id),
        })
    }
}
