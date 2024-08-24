// use std::borrow::BorrowMut;

use candid::{CandidType, Principal};
use ic_cdk::{query, update};
use serde::{Deserialize, Serialize};

use crate::entitymanagement::{self};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct RegisterFarm {
    pub farmer_name: Option<String>,      // Farmer Name
    pub farm_name: Option<String>,        // Farm Name
    pub farm_description: Option<String>, // Farm Description
    pub tags: Option<Vec<String>>,        // Optional list of tags associated with the farm
    pub images: Option<Vec<String>>,      // Optional list of image filenames related to the farm
    pub reports: Option<entitymanagement::Reports>, // Optional reports containing financial and farm-related information
}

// Temporary storage for partial farm data
thread_local! {
    static PARTIAL_FARM_STORAGE: std::cell::RefCell<HashMap<u64, RegisterFarm>> = std::cell::RefCell::new(HashMap::new());
}

#[update]
fn add_farm_chunk(
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
            reports: None,
        });

        // Update the entry with new data
        if let Some(farmer_name) = new_farm_chunk.farmer_name {
            entry.farmer_name = Some(farmer_name);
        }
        if let Some(farm_name) = new_farm_chunk.farm_name {
            entry.farm_name = Some(farm_name);
        }
        if let Some(farm_description) = new_farm_chunk.farm_description {
            entry.farm_description = Some(farm_description);
        }
        if let Some(tags) = new_farm_chunk.tags {
            entry.tags = Some(tags);
        }
        if let Some(images) = new_farm_chunk.images {
            entry.images = Some(images);
        }
        if let Some(reports) = new_farm_chunk.reports {
            entry.reports = Some(reports);
        }

        // Check if the entry is complete
        if entry.farmer_name.is_some() && entry.farm_name.is_some() && entry.farm_description.is_some() {
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
                reports: entry.reports.clone(),
            };

            entitymanagement::FARMER_STORAGE
                .with(|farmers| farmers.borrow_mut().insert(farm_id, complete_farm.clone()));

            entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE
                .with(|farmers| farmers.borrow_mut().insert(farm_id, complete_farm));

            // Remove from partial storage
            storage.remove(&farm_id);

            Ok(entitymanagement::Success::FarmsAgriBizRegisteredSuccesfully {
                msg: format!(
                    "Farm added successfully to the agribusiness: {}",
                    agribusiness_name
                ),
            })
        } else {
            // Entry is not complete yet
            Ok(entitymanagement::Success::PartialDataStored {
                msg: "Partial data stored successfully.".to_string(),
            })
        }
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
