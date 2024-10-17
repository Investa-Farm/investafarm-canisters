use candid::{CandidType, Principal};
use ic_cdk::update;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::entitymanagement::{ self, check_entity_type, EntityType }; 

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    file_id: u64, 
    filename: String,
    agribusiness_name: String,
    principal_id: Principal, 
    farms_uploaded: bool
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum Success {
    FileUploaded { msg: String },
    FarmCreatedSuccessfully { msg: String },
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum Error {
    UploadFailed { msg: String },
    NotAuthorized { msg: String }, 
    FieldEmpty { msg: String },
    FarmNameTaken { msg: String }
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
}

#[update]
// fn upload_agribusiness_spreadsheet(filename: String, file_data: Vec<u8>, agribusiness_name: String) -> Result<Success, Error> {
fn upload_agribusiness_spreadsheet(filename: String, file_data: Vec<u8>, agribusiness_name: String)  {

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
            farms_uploaded: false
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
                info_storage.borrow()
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
        },
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
                storage.iter()
                    .find(|file_info| file_info.file_id == file_id)
                    .map(|file_info| file_info.farms_uploaded)
            });
            
            match file_status {
                Some(status) => Ok(status),
                None => Err(Error::UploadFailed { 
                    msg: format!("No file found with ID '{}'", file_id) 
                })
            }
        },
        _ => Err(Error::NotAuthorized {
            msg: "Only registered farms agribusinesses can check file status".to_string(),
        })
    }
}

// Add a new function to mark file as complete
#[update]
pub fn mark_file_complete(file_id: u64) -> Result<Success, Error> {
    let caller = ic_cdk::caller();
    
    match check_entity_type() {
        EntityType::FarmsAgriBusiness => {
            FILE_INFO_STORAGE.with(|info_storage| {
                let mut info_storage = info_storage.borrow_mut();
                if let Some(file_info) = info_storage.iter_mut().find(|info| 
                    info.file_id == file_id && info.principal_id == caller
                ) {
                    file_info.farms_uploaded = true;
                    Ok(Success::FileUploaded {
                        msg: "File marked as completely processed".to_string(),
                    })
                } else {
                    Err(Error::UploadFailed {
                        msg: "File not found or unauthorized".to_string(),
                    })
                }
            })
        },
        _ => Err(Error::NotAuthorized {
            msg: "Only registered farms agribusinesses can mark files as complete".to_string(),
        })
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
                        verified: false,
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

                    entitymanagement::FARMER_STORAGE.with(|farmers| farmers.borrow_mut().insert(id, farmer.clone()));

                    Ok(Success::FarmCreatedSuccessfully {
                        msg: format!("Farm '{}' has been created successfully", farmer.farm_name),
                    })
                },
                Err(_) => Err(Error::NotAuthorized {
                    msg: "Only registered farms agribusinesses can add farms".to_string(),
                }),
            }
        }
        Err(e) => return Err(e), // Propagate any errors from check_file_status
    }

}