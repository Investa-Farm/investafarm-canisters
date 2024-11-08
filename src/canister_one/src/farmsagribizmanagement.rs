use crate::entitymanagement::{self, check_entity_type, EntityType};
use candid::{CandidType, Principal};
use ic_cdk::{query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    file_id: u64,
    filename: String,
    agribusiness_name: String,
    principal_id: Principal,
    farms_uploaded: bool,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum Success {
    FileUploaded { msg: String },
    FarmCreatedSuccessfully { msg: String },
    FarmDeletedSuccessfully { msg: String },
    FarmVerificationStatusChanged { msg: String },
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum Error {
    UploadFailed { msg: String },
    NotAuthorized { msg: String },
    FieldEmpty { msg: String },
    FarmNameTaken { msg: String },
    FarmNotFound { msg: String },
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct NewFarmer {
    pub farmer_name: String,
    pub farm_name: String,
    pub farm_description: String,
}

thread_local! {
    static NEXT_FILE_ID: RefCell<u64> = RefCell::new(1);
    static AGRIBIZ_FILE_STORAGE: RefCell<HashMap<String, Vec<u8>>> = RefCell::new(HashMap::new());
    static FILE_INFO_STORAGE: RefCell<Vec<FileInfo>> = RefCell::new(Vec::new());
    static FARM_IMAGES: RefCell<HashMap<u64, Vec<Vec<u8>>>> = RefCell::new(HashMap::new());
}

#[update]
// fn upload_agribusiness_spreadsheet(filename: String, file_data: Vec<u8>, agribusiness_name: String) -> Result<Success, Error> {
fn upload_agribusiness_spreadsheet(
    filename: String,
    file_data: Vec<u8>,
    agribusiness_name: String,
) {
    // Generate a new unique file ID
    let file_id = NEXT_FILE_ID.with(|id| {
        let current_id = *id.borrow();
        *id.borrow_mut() = current_id + 1;
        current_id
    });

    AGRIBIZ_FILE_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.insert(filename.clone(), file_data);
    });

    FILE_INFO_STORAGE.with(|info_storage| {
        let mut info_storage = info_storage.borrow_mut();
        info_storage.push(FileInfo {
            file_id,
            filename: filename.clone(),
            agribusiness_name,
            principal_id: ic_cdk::caller(),
            farms_uploaded: false,
        });
    });

    // let entity_type = check_entity_type();

    // match entity_type {
    //     EntityType::SupplyAgriBusiness | EntityType::FarmsAgriBusiness => {
    //         AGRIBIZ_FILE_STORAGE.with(|storage| {
    //             let mut storage = storage.borrow_mut();
    //             storage.insert(filename.clone(), file_data);
    //         });

    //         FILE_INFO_STORAGE.with(|info_storage| {
    //         let mut info_storage = info_storage.borrow_mut();
    //         info_storage.push(FileInfo {
    //             filename: filename.clone(),
    //             agribusiness_name,
    //             principal_id: ic_cdk::caller()
    //         });
    // });

    //         Ok(Success::FileUploaded {
    //             msg: format!("File '{}' uploaded successfully", filename),
    //         })
    //     },
    //     _ => Err(Error::NotAuthorized {
    //         msg: "Only registered agribusinesses can upload files".to_string(),
    //     }),
    // }
}

#[update]
fn get_uploaded_files() -> Result<(Vec<FileInfo>, HashMap<String, Vec<u8>>), Error> {
    let entity_type = check_entity_type();
    let caller = ic_cdk::caller();

    match entity_type {
        EntityType::FarmsAgriBusiness => {
            // Filter FileInfo by caller
            let filtered_file_info = FILE_INFO_STORAGE.with(|info_storage| {
                info_storage
                    .borrow()
                    .iter()
                    .filter(|file_info| file_info.principal_id == caller)
                    .cloned()
                    .collect::<Vec<FileInfo>>()
            });

            // Get only the filenames that belong to the caller
            let caller_filenames: Vec<String> = filtered_file_info
                .iter()
                .map(|info| info.filename.clone())
                .collect();

            // Filter the file contents to only include caller's files
            let filtered_agribiz_files = AGRIBIZ_FILE_STORAGE.with(|agribiz_storage| {
                let storage = agribiz_storage.borrow();
                storage
                    .iter()
                    .filter(|(filename, _)| caller_filenames.contains(filename))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<HashMap<String, Vec<u8>>>()
            });

            if filtered_file_info.is_empty() {
                Ok((Vec::new(), HashMap::new()))
            } else {
                Ok((filtered_file_info, filtered_agribiz_files))
            }
        }
        _ => Err(Error::NotAuthorized {
            msg: "Only registered agribusinesses can view uploaded files".to_string(),
        }),
    }
}

pub fn check_file_status(file_id: u64) -> Result<bool, Error> {
    let entity_type = check_entity_type();

    match entity_type {
        EntityType::FarmsAgriBusiness => {
            let file_status = FILE_INFO_STORAGE.with(|info_storage| {
                let storage = info_storage.borrow();
                storage
                    .iter()
                    .find(|file_info| file_info.file_id == file_id)
                    .map(|file_info| file_info.farms_uploaded)
            });

            match file_status {
                Some(status) => Ok(status),
                None => Err(Error::UploadFailed {
                    msg: format!("No file found with ID '{}'", file_id),
                }),
            }
        }
        _ => Err(Error::NotAuthorized {
            msg: "Only registered farms agribusinesses can check file status".to_string(),
        }),
    }
}
// Add a new function to mark file as complete
#[update]
pub fn mark_file_complete(file_id: u64) -> Result<Success, Error> {
    let caller = ic_cdk::caller();

    match check_entity_type() {
        EntityType::FarmsAgriBusiness => FILE_INFO_STORAGE.with(|info_storage| {
            let mut info_storage = info_storage.borrow_mut();
            if let Some(file_info) = info_storage
                .iter_mut()
                .find(|info| info.file_id == file_id && info.principal_id == caller)
            {
                file_info.farms_uploaded = true;
                Ok(Success::FileUploaded {
                    msg: "File marked as completely processed".to_string(),
                })
            } else {
                Err(Error::UploadFailed {
                    msg: "File not found or unauthorized".to_string(),
                })
            }
        }),
        _ => Err(Error::NotAuthorized {
            msg: "Only registered farms agribusinesses can mark files as complete".to_string(),
        }),
    }
}

#[update]
pub fn register_single_farm(new_farmer: NewFarmer, file_id: u64) -> Result<Success, Error> {
    let caller = ic_cdk::caller();

    // First check if files have already been uploaded
    match check_file_status(file_id) {
        Ok(true) => {
            return Err(Error::UploadFailed {
                msg: "Files have already been uploaded for this agribusiness".to_string(),
            });
        }
        Ok(false) => {
            match entitymanagement::display_specific_farm_agribusiness(caller) {
                Ok(_agribusiness) => {
                    // Validate that all required fields are filled
                    if new_farmer.farmer_name.is_empty()
                        || new_farmer.farm_name.is_empty()
                        || new_farmer.farm_description.is_empty()
                    {
                        return Err(Error::FieldEmpty {
                            msg: "Kindly ensure all required fields are filled!".to_string(),
                        });
                    }

                    // Generate a unique farmer ID
                    let id = entitymanagement::FARMER_STORAGE.with(|farmers| {
                        let farmers = farmers.borrow_mut();
                        let new_id = farmers.len() as u64 + 1;
                        new_id
                    });

                    // Create a new farmer instance
                    let farmer = entitymanagement::Farmer {
                        id,
                        principal_id: Principal::anonymous(),
                        farmer_name: new_farmer.farmer_name,
                        farm_name: new_farmer.farm_name.clone(),
                        farm_description: new_farmer.farm_description,
                        token_collateral: None,
                        farm_assets: None,
                        tags: Some(Vec::new()),
                        amount_invested: None,
                        investors_ids: Principal::anonymous(),
                        verified: true,
                        agri_business: ic_cdk::caller().to_string(),
                        insured: None,
                        publish: true,
                        ifarm_tokens: None,
                        credit_score: None,
                        current_loan_ask: None,
                        loaned: false,
                        loan_maturity: None,
                        time_for_funding_round_to_expire: None,
                        funding_round_start_time: None,
                        loan_start_time: None,
                        images: None,
                        farm_reports: None,
                        financial_reports: None,
                    };

                    // Store the new farmer
                    entitymanagement::REGISTERED_FARMERS.with(|farmers| {
                        farmers
                            .borrow_mut()
                            .insert(farmer.farm_name.clone(), farmer.clone())
                    });

                    entitymanagement::FARMER_STORAGE
                        .with(|farmers| farmers.borrow_mut().insert(id, farmer.clone()));

                    Ok(Success::FarmCreatedSuccessfully {
                        msg: format!("Farm '{}' has been created successfully", farmer.farm_name),
                    })
                }
                Err(_) => Err(Error::NotAuthorized {
                    msg: "Only registered farms agribusinesses can add farms".to_string(),
                }),
            }
        }
        Err(e) => return Err(e), // Propagate any errors from check_file_status
    }
}

#[update]
pub fn delete_single_farm(farm_id: u64) -> Result<Success, Error> {
    // Check if the caller is allowed to delete farms
    // if !is_allowed_principal() {
    //     return Err(Error::PermissionDenied {
    //         msg: "You are not an admin!".to_string(),
    //     });
    // }

    // Retrieve the farmers and attempt to find the one matching the farm_id
    // let farmers = entitymanagement::return_farmers();
    // if let Some(farmer) = farmers.iter().find(|f| f.id == farm_id) {
    //     let farm_name = farmer.farm_name.clone();

    //     // Remove the farm from FARMER_STORAGE
    //     entitymanagement::FARMER_STORAGE.with(|storage| {
    //         storage.borrow_mut().remove(&farm_id);
    //     });

    //     // Remove the farm from REGISTERED_FARMERS
    //     entitymanagement::REGISTERED_FARMERS.with(|registered_farmers| {
    //         registered_farmers.borrow_mut().remove(&farm_name);
    //     });

    //     Ok(Success::FarmDeletedSuccessfully {
    //         msg: format!("Farm with ID '{}' has been deleted successfully", farm_id),
    //     })
    // } else {
    //     Err(Error::FarmNotFound {
    //         msg: format!("Farm with ID '{}' doesn't exist", farm_id),
    //     })
    // }
    let mut farmers = entitymanagement::return_farmers();

    if let Some(_farmer) = farmers.iter_mut().find(|f| f.id == farm_id) {
        entitymanagement::FARMER_STORAGE.with(|storage| {
            storage.borrow_mut().remove(&farm_id);
        });

        // entitymanagement::REGISTERED_FARMERS.with(|registered_farmers| {
        //     registered_farmers.borrow_mut().remove(&farm_id);
        // });

        Ok(Success::FarmDeletedSuccessfully {
            msg: format!("Farm with ID '{}' has been deleted successfully", farm_id),
        })
    } else {
        Err(Error::FarmNotFound {
            msg: format!("Farm with ID '{}' doesn't exist", farm_id),
        })
    }
}

#[update]
pub fn change_verification_status(farm_id: u64, new_status: bool) -> Result<Success, Error> {
    match check_entity_type() {
        EntityType::FarmsAgriBusiness => {
            let mut success = false;
            let mut farm_name = String::new();

            entitymanagement::FARMER_STORAGE.with(|storage| {
                if let Some(farmer) = storage.borrow().get(&farm_id) {
                    let mut updated_farmer = farmer.clone();
                    updated_farmer.verified = new_status;
                    farm_name = updated_farmer.farm_name.clone();
                    success = storage
                        .borrow_mut()
                        .insert(farm_id, updated_farmer)
                        .is_some();

                    // Also update in REGISTERED_FARMERS
                    entitymanagement::REGISTERED_FARMERS.with(|farmers| {
                        if let Some(registered_farmer) = farmers.borrow().get(&farm_name) {
                            let mut updated_registered_farmer = registered_farmer.clone();
                            updated_registered_farmer.verified = new_status;
                            farmers
                                .borrow_mut()
                                .insert(farm_name.clone(), updated_registered_farmer);
                        }
                    });
                }
            });

            if success {
                Ok(Success::FarmVerificationStatusChanged {
                    msg: format!(
                        "Verification status for farm with ID '{}' has been changed to {}",
                        farm_id, new_status
                    ),
                })
            } else {
                Err(Error::FarmNotFound {
                    msg: format!("Farm with ID '{}' not found", farm_id),
                })
            }
        }
        _ => Err(Error::NotAuthorized {
            msg: "Only registered farms agribusinesses can change farm verification status"
                .to_string(),
        }),
    }
}

#[update]
fn add_farm_images(farm_id: u64, images: Vec<Vec<u8>>) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller();

    // First verify the farm exists and belongs to the caller
    let farm = entitymanagement::FARMER_STORAGE
        .with(|storage| storage.borrow().get(&farm_id).clone())
        .ok_or_else(|| entitymanagement::Error::NotAuthorized {
            msg: format!("Farm with ID {} not found", farm_id),
        })?;

    if farm.principal_id != caller {
        return Err(entitymanagement::Error::NotAuthorized {
            msg: "You are not authorized to modify this farm.".to_string(),
        });
    }

    // Merge existing images with new ones
    FARM_IMAGES.with(|images_storage| {
        let mut storage = images_storage.borrow_mut();
        let existing_images = storage.entry(farm_id).or_insert(Vec::new());
        existing_images.extend(images.clone());
    });

    // Generate image references for all images
    let image_refs: Vec<String> = FARM_IMAGES.with(|images_storage| {
        let storage = images_storage.borrow();
        let total_images = storage.get(&farm_id).map(|imgs| imgs.len()).unwrap_or(0);
        (0..total_images).map(|index| format!("image_{}", index)).collect()
    });

    // Update farm with all image references
    let mut updated_farm = farm;
    updated_farm.images = Some(image_refs);
    
    entitymanagement::FARMER_STORAGE
        .with(|storage| storage.borrow_mut().insert(farm_id, updated_farm.clone()));

    Ok(entitymanagement::Success::PartialDataStored {
        msg: "Images added successfully".to_string(),
    })
}
#[update]
fn add_financial_reports(
    farm_id: u64,
    financial_reports: Option<Vec<entitymanagement::FinancialReport>>,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller();

    let mut farms_for_agribusiness = get_farms_for_agribusiness();

    if let Some(farm) = farms_for_agribusiness
        .iter_mut()
        .find(|f| f.id == farm_id && f.principal_id == caller)
    {
        if let Some(reports) = financial_reports {
            farm.financial_reports = Some(reports);
        }

        let farm_clone_1 = farm.clone();
        let farm_clone_2 = farm.clone();

        entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE
            .with(|service| service.borrow_mut().insert(farm_id, farm_clone_1));

        entitymanagement::FARMER_STORAGE
            .with(|service| service.borrow_mut().insert(farm_id, farm_clone_2));

        Ok(entitymanagement::Success::PartialDataStored {
            msg: "Financial reports added successfully.".to_string(),
        })
    } else {
        Err(entitymanagement::Error::NotAuthorized {
            msg: format!("Farm not found!"),
        })
    }
}

#[update]
fn add_farm_reports(
    farm_id: u64,
    farm_reports: Option<Vec<entitymanagement::FarmReport>>,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller();

    let mut farms_for_agribusiness = get_farms_for_agribusiness();

    if let Some(farm) = farms_for_agribusiness
        .iter_mut()
        .find(|f| f.id == farm_id && f.principal_id == caller)
    {
        if let Some(reports) = farm_reports {
            farm.farm_reports = Some(reports);
        }

        let farm_clone_1 = farm.clone();
        let farm_clone_2 = farm.clone();

        entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE
            .with(|service| service.borrow_mut().insert(farm_id, farm_clone_1));

        entitymanagement::FARMER_STORAGE
            .with(|service| service.borrow_mut().insert(farm_id, farm_clone_2));

        Ok(entitymanagement::Success::PartialDataStored {
            msg: "Farm reports added successfully.".to_string(),
        })
    } else {
        Err(entitymanagement::Error::NotAuthorized {
            msg: format!("Farm not found!"),
        })
    }
}

#[query]
fn get_farms_for_agribusiness() -> Vec<entitymanagement::Farmer> {
    entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE.with(|farms| {
        farms
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect()
    })
}

#[update]
fn publish_unpublish(
    farm_id: u64,
    publish: bool,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller();

    let mut farms_for_agribusiness = get_farms_for_agribusiness();

    if let Some(farm) = farms_for_agribusiness
        .iter_mut()
        .find(|f| f.principal_id == caller)
    {
        farm.publish = publish;

        let farm_clone_1 = farm.clone();
        let farm_clone_2 = farm.clone();

        entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE
            .with(|service| service.borrow_mut().insert(farm_id, farm_clone_1));

        entitymanagement::FARMER_STORAGE
            .with(|service| service.borrow_mut().insert(farm_id, farm_clone_2));

        Ok(entitymanagement::Success::FarmPublishedSuccesfully {
            msg: format!(
                "Farm publish status succesfully updated to {}",
                farm.publish
            ),
        })
    } else {
        Err(entitymanagement::Error::NotAuthorized {
            msg: format!("Farm not found!"),
        })
    }
}

#[update]
fn delete_farm(farm_id: u64) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller();

    let mut farms_for_agribusiness = get_farms_for_agribusiness();

    if let Some(index) = farms_for_agribusiness
        .iter()
        .position(|f| f.id == farm_id && f.principal_id == caller)
    {
        let farm = farms_for_agribusiness.remove(index);

        // Remove the farm from FARMS_FOR_AGRI_BUSINESS
        entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE.with(|farms| {
            let mut farms = farms.borrow_mut();
            farms.remove(&farm_id);
        });

        // Remove the farm from FARMER_STORAGE
        entitymanagement::FARMER_STORAGE.with(|farmers| {
            let mut farmers = farmers.borrow_mut();
            farmers.remove(&farm_id);
        });

        Ok(entitymanagement::Success::FarmDeletedSuccesfully {
            msg: format!("Farm {} has been deleted succesfully", farm.farm_name),
        })
    } else {
        Err(entitymanagement::Error::ErrorOccured {
            msg: format!("An error occured!"),
        })
    }
}
#[query]
fn get_farm_images(farm_id: u64) -> Result<Vec<Vec<u8>>, entitymanagement::Error> {
    let _caller = ic_cdk::caller();

    // First verify the farm exists
    let _farm = entitymanagement::FARMER_STORAGE
        .with(|storage| storage.borrow().get(&farm_id).clone())
        .ok_or_else(|| entitymanagement::Error::NotAuthorized {
            msg: format!("Farm with ID {} not found", farm_id),
        })?;

    // Get images from storage
    let images = FARM_IMAGES.with(|images_storage| {
        images_storage
            .borrow()
            .get(&farm_id)
            .cloned()
            .unwrap_or_default()
    });

    if images.is_empty() {
        return Err(entitymanagement::Error::ErrorOccured {
            msg: format!("No images found for farm {}", farm_id)
        });
    }

    Ok(images)
}
