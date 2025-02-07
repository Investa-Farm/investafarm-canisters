use crate::farmerfiles::FarmerReportVec;
use crate::{farmerfiles, farmsagribizmanagement::FARM_IMAGES};
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
    FarmNotFound { msg: String },
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
        Principal::from_text("3ut3n-6rt35-45boi-6vidq-fur7n-5jlvg-wggto-dqwcr-gwtuk-rst7y-zae").unwrap(),
        Principal::from_text("7eeri-bkt2d-qcqc3-7w5cv-etdbs-mulun-napzl-alxdp-7dj4j-qbbfz-cqe").unwrap(), 
        Principal::from_text("jfsrf-v3jew-2j7c5-z2v5l-cpib7-7h66c-y2axw-ybf4m-qmljz-coz7i-6ae").unwrap(), 
        Principal::from_text("gpjen-s3wpj-cuhjh-w3qhk-d662f-zuqhs-cp4bq-oz2hw-aqbu4-nzg6y-kqe").unwrap()
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

#[update]
fn manual_verify_entity(entity_type: String, id: u64, verified: bool) -> Result<(), Error> {
    // Ensure only admins can manually verify entities
    if !is_allowed_principal() {
        return Err(Error::PermissionDenied { 
            msg: String::from("Caller must be an admin to manually verify entities")
        });
    }

    match entity_type.as_str() {
        "farmer" => {
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
        },
        "investor" => {
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
        },
        "supply_agribusiness" => {
            let mut supply_agribusinesses = entitymanagement::return_supply_agribusiness();
            if let Some(agribiz) = supply_agribusinesses.iter_mut().find(|s| s.id == id) {
                agribiz.verified = verified;
                entitymanagement::SUPPLY_AGRIBUSINESS_STORAGE
                    .with(|service| service.borrow_mut().insert(id, agribiz.clone()));
                Ok(())
            } else {
                Err(Error::SupplyAgriBizNotFound {
                    msg: format!("Supply agribusiness with id {} doesn't exist", id),
                })
            }
        },
        "farms_agribusiness" => {
            let mut farms_agribusinesses = entitymanagement::return_farms_agribusiness();
            if let Some(agribiz) = farms_agribusinesses.iter_mut().find(|s| s.id == id) {
                agribiz.verified = verified;
                entitymanagement::FARMS_AGRIBUSINESS_STORAGE
                    .with(|service| service.borrow_mut().insert(id, agribiz.clone()));
                Ok(())
            } else {
                Err(Error::FarmsAgriBizNotFound {
                    msg: format!("Farms agribusiness with id {} doesn't exist", id),
                })
            }
        },
        _ => Err(Error::PermissionDenied {
            msg: String::from("Invalid entity type provided"),
        })
    }
}

// Functionality for removing farm images and farm reports
#[update]
pub fn admin_remove_farm_image(farm_id: u64, image_index: usize) -> Result<(), Error> {
    // if !is_allowed_principal() {
    //     return Err(Error::PermissionDenied { 
    //         msg: String::from("Only admins can remove farm images") 
    //     });
    // }

    // Get existing images first
    let mut images = FARM_IMAGES.with(|images_storage| {
        images_storage
            .borrow()
            .get(&farm_id)
            .ok_or_else(|| Error::FarmNotFound {
                msg: format!("No images found for farm {}", farm_id)
            })
    })?;

    // Validate index before making any changes
    if image_index >= images.0.len() {
        return Err(Error::InvalidJobId { 
            msg: format!("Image index {} out of bounds", image_index) 
        });
    }

    // Remove the image
    images.0.remove(image_index);
    
    // Update FARM_IMAGES storage
    let farm_images_update = FARM_IMAGES.with(|images_storage| {
        images_storage.borrow_mut().insert(farm_id, images.clone())
    });

    if farm_images_update.is_none() {
        return Err(Error::FarmNotFound {
            msg: format!("Failed to update farm images for farm {}", farm_id)
        });
    }

    // Update farmer storage with error handling
    let farmer_update_result = entitymanagement::FARMER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut farmer) = storage.get(&farm_id) {
            if let Some(refs) = &mut farmer.images {
                refs.remove(image_index);
                storage.insert(farm_id, farmer);
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    if !farmer_update_result {
        return Err(Error::FarmNotFound {
            msg: format!("Failed to update farmer record for farm {}", farm_id)
        });
    }

    Ok(())
}

#[update]
pub fn admin_remove_farm_report(farm_id: u64, report_index: usize) -> Result<(), Error> {
    
    // Delete the specific file from FILE_STORAGE
    let file_deleted = entitymanagement::FILE_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage
            .iter()
            .filter(|(k, _)| k.0.starts_with(&format!("report_{}", farm_id)))
            .nth(report_index)
            .map(|(k, _)| storage.remove(&k))
            .is_some()
    });

    if !file_deleted {
        return Err(Error::FarmNotFound {
            msg: format!("Failed to delete file for farm report {}", report_index)
        });
    } 

    farmerfiles::FARM_REPORTS.with(|reports| {
        let mut reports = reports.borrow_mut();
        if let Some(report_vec) = reports.get(&farm_id) {
            let mut updated_vec = report_vec.0.clone();
            if report_index < updated_vec.len() {
                updated_vec.remove(report_index);
                reports.insert(farm_id, FarmerReportVec(updated_vec));

                // Update farmer storage
                entitymanagement::FARMER_STORAGE.with(|storage| {
                    let mut storage = storage.borrow_mut();
                    if let Some(mut farmer) = storage.get(&farm_id) {
                        if let Some(farmer_reports) = &mut farmer.farm_reports {
                            farmer_reports.remove(report_index);
                            storage.insert(farm_id, farmer);
                        }
                    }
                });

                // Update agribusiness storage
                entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE.with(|agri_storage| {
                    let mut agri_storage = agri_storage.borrow_mut();
                    if let Some(mut agri_farmer) = agri_storage.get(&farm_id) {
                        if let Some(agri_reports) = &mut agri_farmer.farm_reports {
                            agri_reports.remove(report_index);
                            agri_storage.insert(farm_id, agri_farmer);
                        }
                    }
                });

                Ok(())
            } else {
                Err(Error::InvalidJobId { 
                    msg: format!("Report index {} out of bounds", report_index) 
                })
            }
        } else {
            Err(Error::FarmNotFound { 
                msg: format!("Reports not found for farm {}", farm_id) 
            })
        }
    })
}