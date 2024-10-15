use candid::{CandidType, Principal};
use ic_cdk::update;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::entitymanagement::{ self, check_entity_type, EntityType }; 

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    filename: String,
    agribusiness_name: String,
    principal_id: Principal
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
    static AGRIBIZ_FILE_STORAGE: RefCell<HashMap<String, Vec<u8>>> = RefCell::new(HashMap::new());
    static FILE_INFO_STORAGE: RefCell<Vec<FileInfo>> = RefCell::new(Vec::new());
}

#[update]
// fn upload_agribusiness_spreadsheet(filename: String, file_data: Vec<u8>, agribusiness_name: String) -> Result<Success, Error> {
fn upload_agribusiness_spreadsheet(filename: String, file_data: Vec<u8>, agribusiness_name: String)  {
    AGRIBIZ_FILE_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        storage.insert(filename.clone(), file_data);
    });

    FILE_INFO_STORAGE.with(|info_storage| {
        let mut info_storage = info_storage.borrow_mut();
        info_storage.push(FileInfo {
            filename: filename.clone(),
            agribusiness_name,
            principal_id: ic_cdk::caller()
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

    match entity_type {
        EntityType::FarmsAgriBusiness => {
            let file_info = FILE_INFO_STORAGE.with(|info_storage| {
                info_storage.borrow().clone()
            });
            let agribiz_file_info = AGRIBIZ_FILE_STORAGE.with(|agribiz_storage| {
                agribiz_storage.borrow().clone()
            });
            Ok((file_info, agribiz_file_info))
        },
        _ => Err(Error::NotAuthorized {
            msg: "Only registered agribusinesses can view uploaded files".to_string(),
        }),
    }
}

#[update]
pub fn register_single_farm(new_farmer: NewFarmer) -> Result<Success, Error> {
    // let entity_type = check_entity_type();

    // match entity_type {
    //     EntityType::FarmsAgriBusiness => {
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
        // },
        // _ => Err(Error::NotAuthorized {
        //     msg: "Only registered farms agribusinesses can add farms".to_string(),
        // }),
    }
// }