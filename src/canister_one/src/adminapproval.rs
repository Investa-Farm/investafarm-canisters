use crate::entitymanagement;
use candid::{CandidType, Principal};
use ic_cdk::{update, caller, query};
use serde::{Deserialize, Serialize};

// Error Messages
#[derive(CandidType, Deserialize, Serialize)]
pub enum Error {
    FarmerNotFound { msg: String },
    PermissionDenied { msg: String },
    InvestorNotFound { msg: String },
    SupplyAgriBizNotFound { msg: String },
    FarmsAgriBizNotFound { msg: String },
    InvalidJobId { msg: String },
}

// Helper function to check if caller is an admin or registered entity
fn is_authorized(entity_type: &str) -> bool {
    let caller_id = caller();
    
    // First check if caller is an admin
    if is_allowed_principal() {
        return true;
    }

    // Then check if caller is a registered entity of the specified type
    match entity_type {
        "farmer" => {
            let farmers = entitymanagement::return_farmers();
            farmers.iter().any(|f| f.principal_id == caller_id && f.verified)
        },
        "investor" => {
            let investors = entitymanagement::return_investors();
            investors.iter().any(|i| i.principal_id == caller_id && i.verified)
        },
        "supply_agribusiness" => {
            let businesses = entitymanagement::return_supply_agribusiness();
            businesses.iter().any(|b| b.principal_id == caller_id && b.verified)
        },
        "farms_agribusiness" => {
            let businesses = entitymanagement::return_farms_agribusiness();
            businesses.iter().any(|b| b.principal_id == caller_id && b.verified)
        },
        _ => false
    }
}

// Admin callers
#[query]
pub fn is_allowed_principal() -> bool {
    let allowed_principals = vec![
        Principal::from_text("u6mjj-6nldg-axc2d-yhwxu-324vw-aq4s2-n4l35-boxrh-4rnbn-qyz4m-pae").unwrap(),
        Principal::from_text("grvsb-a7n2k-5ddft-lyfah-kl62t-ir2ih-4zvsc-ti5sf-qxboa-5f4zk-oae").unwrap(), 
        Principal::from_text("ipd2t-z274n-iv4hx-ravmg-7yq3w-ownym-5zwnb-cfuu4-ayo4s-k7tp5-jae").unwrap(), 
        Principal::from_text("3ut3n-6rt35-45boi-6vidq-fur7n-5jlvg-wggto-dqwcr-gwtuk-rst7y-zae").unwrap()
    ];

    let caller_principal = caller();

    allowed_principals.contains(&caller_principal)
}

#[update]
pub fn verify_farmer(id: u64, verified: bool, kyc_job_id: String) -> Result<(), Error> {
    // Validate job_id is not empty
    if kyc_job_id.trim().is_empty() {
        return Err(Error::InvalidJobId { 
            msg: String::from("KYC job ID cannot be empty") 
        });
    }

    if !is_authorized("farmer") {
        return Err(Error::PermissionDenied { 
            msg: String::from("Caller must be an admin or a verified farmer") 
        });
    }

    let mut farmers = entitymanagement::return_farmers();
    if let Some(farmer) = farmers.iter_mut().find(|f| f.id == id) {
        farmer.verified = verified;
        // Add the KYC job ID to the farmer struct
        // Note: You'll need to add a kyc_job_id field to your Farmer struct
        farmer.kyc_job_id = Some(kyc_job_id);
        
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
pub fn verify_investor(id: u64, verified: bool, kyc_job_id: String) -> Result<(), Error> {
    if kyc_job_id.trim().is_empty() {
        return Err(Error::InvalidJobId { 
            msg: String::from("KYC job ID cannot be empty") 
        });
    }

    if !is_authorized("investor") {
        return Err(Error::PermissionDenied { 
            msg: String::from("Caller must be an admin or a verified investor") 
        });
    }

    let mut investors = entitymanagement::return_investors();
    if let Some(investor) = investors.iter_mut().find(|i| i.id == id) {
        investor.verified = verified;
        // Add the KYC job ID to the investor struct
        investor.kyc_job_id = Some(kyc_job_id);
        
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
pub fn verify_supply_agribusiness(id: u64, verified: bool, kyc_job_id: String) -> Result<(), Error> {
    if kyc_job_id.trim().is_empty() {
        return Err(Error::InvalidJobId { 
            msg: String::from("KYC job ID cannot be empty") 
        });
    }

    if !is_authorized("supply_agribusiness") {
        return Err(Error::PermissionDenied { 
            msg: String::from("Caller must be an admin or a verified supply agribusiness") 
        });
    }

    let mut supply_agribusinesses = entitymanagement::return_supply_agribusiness();
    if let Some(agribiz) = supply_agribusinesses.iter_mut().find(|s| s.id == id) {
        agribiz.verified = verified;
        // Add the KYC job ID to the agribusiness struct
        agribiz.kyc_job_id = Some(kyc_job_id);
        
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
pub fn verify_farms_agribusiness(id: u64, verified: bool, kyc_job_id: String) -> Result<(), Error> {
    if kyc_job_id.trim().is_empty() {
        return Err(Error::InvalidJobId { 
            msg: String::from("KYC job ID cannot be empty") 
        });
    }

    if !is_authorized("farms_agribusiness") {
        return Err(Error::PermissionDenied { 
            msg: String::from("Caller must be an admin or a verified farms agribusiness") 
        });
    }

    let mut farms_agribusinesses = entitymanagement::return_farms_agribusiness();
    if let Some(agribiz) = farms_agribusinesses.iter_mut().find(|s| s.id == id) {
        agribiz.verified = verified;
        // Add the KYC job ID to the agribusiness struct
        agribiz.kyc_job_id = Some(kyc_job_id);
        
        entitymanagement::FARMS_AGRIBUSINESS_STORAGE
            .with(|service| service.borrow_mut().insert(id, agribiz.clone()));
        Ok(())
    } else {
        Err(Error::FarmsAgriBizNotFound {
            msg: format!("Farmer agri business with id {} doesn't exist", id),
        })
    }
}