use candid::{CandidType, Principal};
use ic_cdk::update;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::entitymanagement::{ check_entity_type, EntityType }; 

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    filename: String,
    agribusiness_name: String,
    principal_id: Principal
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum Success {
    FileUploaded { msg: String },
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum Error {
    UploadFailed { msg: String },
    NotAuthorized { msg: String }
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