// use std::borrow::BorrowMut;

use candid::{CandidType, Principal};
use ic_cdk::{query, update};
use serde::{Deserialize, Serialize};

use crate::entitymanagement::{self};
use std::{cell::RefCell, collections::HashMap};

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct RegisterFarm {
    pub farmer_name: Option<String>,      // Farmer Name
    pub farm_name: Option<String>,        // Farm Name
    pub farm_description: Option<String>, // Farm Description
    pub tags: Option<Vec<String>>,        // Optional list of tags associated with the farm
    pub images: Option<Vec<String>>,      // Optional list of image filenames related to the farm
    // pub reports: Option<entitymanagement::Reports>, // Optional reports containing financial and farm-related information
    pub financial_reports: Option<Vec<entitymanagement::FinancialReport>>,
    pub farm_reports: Option<Vec<entitymanagement::FarmReport>>,
}

// Temporary storage for partial farm data
thread_local! {
static PARTIAL_FARM_STORAGE: std::cell::RefCell<HashMap<u64, RegisterFarm>> = std::cell::RefCell::new(HashMap::new());
static FARM_IMAGES: RefCell<HashMap<u64, Vec<Vec<u8>>>> = RefCell::new(HashMap::new());}

#[update]
fn register_farm_details(
    farm_id: u64,
    new_farm_chunk: RegisterFarm,
    agribusiness_name: String,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    PARTIAL_FARM_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let entry = storage.entry(farm_id).or_insert(RegisterFarm {
            farmer_name: None,
            farm_name: None,
            farm_description: None,
            tags: None,
            images: None,
            // reports: None,.
            farm_reports: None,
            financial_reports: None,
        });

        // Update the entry with new data
        if let Some(name) = new_farm_chunk.farmer_name {
            entry.farmer_name = Some(name);
        }
        if let Some(name) = new_farm_chunk.farm_name {
            entry.farm_name = Some(name);
        }
        if let Some(description) = new_farm_chunk.farm_description {
            entry.farm_description = Some(description);
        }

        // Check if the entry is complete
        if entry.farmer_name.is_some()
            && entry.farm_name.is_some()
            && entry.farm_description.is_some()
        {
            // Complete entry, move to permanent storage
            let complete_farm = entitymanagement::Farmer {
                id: farm_id,
                principal_id: ic_cdk::caller(),
                farmer_name: entry.farmer_name.clone().unwrap(),
                farm_name: entry.farm_name.clone().unwrap(),
                farm_description: entry.farm_description.clone().unwrap(),
                farm_assets: None,
                amount_invested: None,
                investors_ids: Principal::anonymous(),
                verified: true,
                agri_business: agribusiness_name.clone(),
                insured: None,
                publish: false,
                ifarm_tokens: None,
                credit_score: None,
                current_loan_ask: None,
                loaned: false,
                loan_maturity: None,
                time_for_funding_round_to_expire: None,
                funding_round_start_time: None,
                loan_start_time: None,
                token_collateral: None,
                tags: entry.tags.clone(),
                images: entry.images.clone(),
                financial_reports: entry.financial_reports.clone(),
                farm_reports: entry.farm_reports.clone(),
            };

            entitymanagement::FARMER_STORAGE
                .with(|farmers| farmers.borrow_mut().insert(farm_id, complete_farm.clone()));

            entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE
                .with(|farmers| farmers.borrow_mut().insert(farm_id, complete_farm));

            // Remove from partial storage
            storage.remove(&farm_id);

            Ok(
                entitymanagement::Success::FarmsAgriBizRegisteredSuccesfully {
                    msg: format!(
                        "Farm added successfully to the agribusiness: {}",
                        agribusiness_name
                    ),
                },
            )
        } else {
            // Entry is not complete yet
            Ok(entitymanagement::Success::PartialDataStored {
                msg: "Partial data stored successfully.".to_string(),
            })
        }
    })
}

#[update]
fn add_farm_tags(
    farm_id: u64,
    tags: Option<Vec<String>>,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller();

    // Retrieve the list of farms for the agribusiness
    let mut farms_for_agribusiness = get_farms_for_agribusiness();

    // Find the specific farm using the farm_id and caller's principal ID
    if let Some(farm) = farms_for_agribusiness
        .iter_mut()
        .find(|f| f.id == farm_id && f.principal_id == caller)
    {
        // Update the tags
        if let Some(t) = tags {
            farm.tags = Some(t);
        }

        // Clone the updated farm to store in both storages
        let farm_clone_1 = farm.clone();
        let farm_clone_2 = farm.clone();

        // Update the permanent storage
        entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE
            .with(|service| service.borrow_mut().insert(farm_id, farm_clone_1));

        entitymanagement::FARMER_STORAGE
            .with(|service| service.borrow_mut().insert(farm_id, farm_clone_2));

        // Return success message
        Ok(entitymanagement::Success::PartialDataStored {
            msg: "Tags added successfully.".to_string(),
        })
    } else {
        // Return error if the farm is not found
        Err(entitymanagement::Error::NotAuthorized {
            msg: format!("Farm not found!"),
        })
    }
}
#[update]
fn add_farm_images(
    farm_id: u64,
    images: Vec<Vec<u8>>,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller();

    // Check both storages for the farm
    let farm = entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE
        .with(|storage| storage.borrow().get(&farm_id).clone())
        .or_else(|| {
            entitymanagement::FARMER_STORAGE.with(|storage| storage.borrow().get(&farm_id).clone())
        });

    if let Some(farm) = farm {
        if farm.principal_id == caller {
            FARM_IMAGES.with(|images_storage| {
                let mut images_storage = images_storage.borrow_mut();
                images_storage.entry(farm_id).or_default().extend(images);
            });

            // Update both storages
            entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE
                .with(|service| service.borrow_mut().insert(farm_id, farm.clone()));

            entitymanagement::FARMER_STORAGE
                .with(|service| service.borrow_mut().insert(farm_id, farm));

            Ok(entitymanagement::Success::PartialDataStored {
                msg: "Images added successfully.".to_string(),
            })
        } else {
            Err(entitymanagement::Error::NotAuthorized {
                msg: "You are not authorized to modify this farm.".to_string(),
            })
        }
    } else {
        Err(entitymanagement::Error::NotAuthorized {
            msg: format!("Farm with ID {} not found in either storage!", farm_id),
        })
    }
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
