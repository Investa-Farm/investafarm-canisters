use crate::entitymanagement::{self, check_entity_type, BoundedBytes, BoundedString, EntityType, MEMORY_MANAGER, Memory};
use candid::{CandidType, Principal, Encode, Decode};
use ic_cdk::{query, update};
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::borrow::Cow;
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{BoundedStorable, StableBTreeMap};

// pub type Memory = VirtualMemory<DefaultMemoryImpl>;

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

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImagesBoundedBytes(pub Vec<Vec<u8>>);

impl Storable for FileInfo {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FileInfo {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for ImagesBoundedBytes {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(&self.0).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let images: Vec<Vec<u8>> = Decode!(bytes.as_ref(), Vec<Vec<u8>>).unwrap();
        ImagesBoundedBytes(images)
    }
}

impl BoundedStorable for ImagesBoundedBytes {
    const MAX_SIZE: u32 = 90_024_000;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static NEXT_FILE_ID: RefCell<u64> = RefCell::new(1);
    
    pub static AGRIBIZ_FILE_STORAGE: RefCell<StableBTreeMap<BoundedString, BoundedBytes, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(9)))
        ));

    pub static FILE_INFO_STORAGE: RefCell<StableBTreeMap<u64, FileInfo, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        ));

    pub static FARM_IMAGES: RefCell<StableBTreeMap<u64, ImagesBoundedBytes, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
        ));
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
        storage.insert(entitymanagement::BoundedString(filename.clone()), entitymanagement::BoundedBytes(file_data));
    });

    FILE_INFO_STORAGE.with(|info_storage| {
        let mut info_storage = info_storage.borrow_mut();
        info_storage.insert(file_id, FileInfo {
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
                    .filter(|(_, file_info)| file_info.principal_id == caller)
                    .map(|(_, file_info)| file_info.clone())
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
                    .filter(|(filename, _)| caller_filenames.contains(&filename.0))
                    .map(|(k, v)| (k.0.clone(), v.0.clone()))
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
                    .find(|(id, _)| *id == file_id)
                    .map(|(_, file_info)| file_info.farms_uploaded)
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
                .iter()
                .find(|(id, info)| *id == file_id && info.principal_id == caller)
            {
                let mut updated_info = file_info.1.clone();
                updated_info.farms_uploaded = true;
                info_storage.insert(file_id, updated_info);
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
                        max_loan_amount: None, 
                        current_loan_ask: None,
                        loaned: false,
                        loan_maturity: None,
                        time_for_funding_round_to_expire: None,
                        funding_round_start_time: None,
                        loan_start_time: None,
                        images: None,
                        farm_reports: None,
                        financial_reports: None,
                        kyc_job_id: None, 
                        email: None
                    };

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

    // Verify farm exists and belongs to either the farmer or agribusiness
    let farm = entitymanagement::FARMER_STORAGE
        .with(|storage| storage.borrow().get(&farm_id))
        .ok_or_else(|| entitymanagement::Error::NotAuthorized {
            msg: format!("Farm with ID {} not found", farm_id),
    })?;

    // Check ownership by comparing both farmer principal_id and agri_business with caller
    if farm.principal_id != caller && farm.agri_business != caller.to_string() {
        return Err(entitymanagement::Error::NotAuthorized {
            msg: "Only the farm owner or associated agribusiness can add images".to_string(),
        });
    }

    // Store images for this specific farm
    FARM_IMAGES.with(|images_storage| {
        let mut storage = images_storage.borrow_mut();
        let farm_specific_images = if let Some(existing_images) = storage.get(&farm_id) {
            let mut updated_images = existing_images.clone();
            updated_images.0.extend(images.clone());
            updated_images
        } else {
            ImagesBoundedBytes(images)
        };
        storage.insert(farm_id, farm_specific_images)
    });

    // Generate image references specific to this farm
    let image_refs: Vec<String> = FARM_IMAGES.with(|images_storage| {
        let storage = images_storage.borrow();
        let total_images = storage.get(&farm_id).map(|imgs| imgs.0.len()).unwrap_or(0);
        (0..total_images).map(|index| format!("farm_{}_image_{}", farm_id, index)).collect()
    });

    // Update farm with its specific image references
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

    // Verify farm exists and belongs to this agribusiness
    let farm = entitymanagement::FARMER_STORAGE
        .with(|storage| storage.borrow().get(&farm_id))
        .ok_or_else(|| entitymanagement::Error::NotAuthorized {
            msg: format!("Farm with ID {} not found", farm_id),
        })?;

    // Check ownership by comparing agri_business field with caller
    if farm.agri_business != caller.to_string() {
        return Err(entitymanagement::Error::NotAuthorized {
            msg: "This farm does not belong to your agribusiness".to_string(),
        });
    }

    // Update farm with new reports
    let mut updated_farm = farm.clone();
    updated_farm.farm_reports = farm_reports;

    // Update both storage locations with the farm-specific data
    entitymanagement::FARMER_STORAGE
        .with(|service| service.borrow_mut().insert(farm_id, updated_farm.clone()));

    entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE
        .with(|service| service.borrow_mut().insert(farm_id, updated_farm));

    Ok(entitymanagement::Success::PartialDataStored {
        msg: "Farm reports added successfully.".to_string(),
    })
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
pub fn delete_farm(farm_id: u64) -> Result<Success, Error> {
    let caller = ic_cdk::caller();
    
    // First verify the farm exists
    let farm = entitymanagement::FARMER_STORAGE
        .with(|storage| storage.borrow().get(&farm_id))
        .ok_or_else(|| Error::FarmNotFound {
            msg: format!("Farm with ID {} not found", farm_id)
        })?;

    // Check if caller is either the farm owner or the associated agribusiness
    // if farm.principal_id != caller && farm.agri_business != caller.to_string() {
    //     return Err(Error::NotAuthorized {
    //         msg: "Only the farm owner or associated agribusiness can delete this farm".to_string()
    //     });
    // }

    // Remove from FARMER_STORAGE
    entitymanagement::FARMER_STORAGE.with(|storage| {
        storage.borrow_mut().remove(&farm_id);
    });

    // Remove from FARMS_FOR_AGRIBUSINESS_STORAGE if exists
    entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE.with(|storage| {
        storage.borrow_mut().remove(&farm_id);
    });

    // Remove any associated images
    FARM_IMAGES.with(|storage| {
        storage.borrow_mut().remove(&farm_id);
    });

    Ok(Success::FarmDeletedSuccessfully {
        msg: format!("Farm '{}' has been deleted successfully", farm.farm_name)
    })
}


#[query]
fn get_farm_images(farm_id: u64) -> Result<Vec<Vec<u8>>, entitymanagement::Error> {
    let _caller = ic_cdk::caller();

    // Verify the farm exists
    let _farm = entitymanagement::FARMER_STORAGE
        .with(|storage| storage.borrow().get(&farm_id)) 
        .ok_or_else(|| entitymanagement::Error::NotAuthorized {
            msg: format!("Farm with ID {} not found", farm_id),
        })?;

    // Get images from storage
    let images = FARM_IMAGES.with(|images_storage| {
        images_storage
            .borrow()
            .get(&farm_id)
            .map(|images| images.clone()) // Clone the `ImagesBoundedBytes` value inside the `Option`
    });

    if let Some(images) = images {
        if images.0.is_empty() {
            return Err(entitymanagement::Error::ErrorOccured {
                msg: format!("No images found for farm {}", farm_id),
            });
        }
        return Ok(images.0);
    }

    Err(entitymanagement::Error::ErrorOccured {
        msg: format!("No images found for farm {}", farm_id),
    })
}


#[update]
fn edit_farm(farm_id: u64, updates: FarmUpdates) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller();
    
    // First verify the farm exists and belongs to this agribusiness
    entitymanagement::FARMER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        
        if let Some(mut farm) = storage.get(&farm_id) {
            // Verify ownership through agri_business field
            if farm.agri_business != caller.to_string() {
                return Err(entitymanagement::Error::NotAuthorized {
                    msg: "Only the associated agribusiness can edit this farm".to_string(),
                });
            }

            // Update fields if provided in updates
            if let Some(description) = updates.farm_description {
                farm.farm_description = description;
            }
            if let Some(name) = updates.farm_name {
                farm.farm_name = name;
            }
            if let Some(credit_score) = updates.credit_score {
                farm.credit_score = Some(credit_score);
            }
            if let Some(max_loan) = updates.max_loan_amount {
                farm.max_loan_amount = Some(max_loan);
            }
            if let Some(publish) = updates.publish {
                farm.publish = publish;
            }
            if let Some(verified) = updates.verified {
                farm.verified = verified;
            }
            if let Some(amount) = updates.amount_invested {
                farm.amount_invested = Some(amount);
            } 
            if let Some(loan_ask) = updates.current_loan_ask {
                farm.current_loan_ask = Some(loan_ask);
            }           

            // Save updated farm
            storage.insert(farm_id, farm.clone());
            
            Ok(entitymanagement::Success::FarmUpdateSuccesfull {
                msg: format!("Farm {} updated successfully", farm_id),
            })
        } else {
            Err(entitymanagement::Error::FarmNotFound {
                msg: format!("Farm with ID {} not found", farm_id),
            })
        }
    })
}

// Add this struct to hold optional update fields
#[derive(CandidType, Serialize, Deserialize)]
pub struct FarmUpdates {
    farm_name: Option<String>,
    farm_description: Option<String>, 
    credit_score: Option<u64>,
    max_loan_amount: Option<u64>,
    publish: Option<bool>,
    verified: Option<bool>, 
    amount_invested: Option<u64>,
    current_loan_ask: Option<u64>, 
}