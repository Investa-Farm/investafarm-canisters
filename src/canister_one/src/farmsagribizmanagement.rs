// use std::borrow::BorrowMut;

use candid::Principal;
use ic_cdk::{query, update};

use crate::entitymanagement::{self};

#[update] 
fn add_farm_to_agribusiness(new_farmer: entitymanagement::NewFarmer, agribusiness_name: String) -> Result<entitymanagement::Success, entitymanagement::Error> {
    if new_farmer.farmer_name.is_empty() || new_farmer.farm_name.is_empty() || new_farmer.farm_description.is_empty() {
        return Err(entitymanagement::Error::FieldEmpty { msg: format!("Kindly ensure all required fieilds are filled!") })
    }

    let id = entitymanagement::FARMER_ID.with(|id| entitymanagement::_increament_id(id)); 
     
    let response = entitymanagement::_is_principal_id_registered(ic_cdk::caller()); 

    if let Err(entitymanagement::Error::PrincipalIdAlreadyRegistered { msg: _ }) = response {
        // Code for when the principal ID is already registered
        let farmer = entitymanagement::Farmer {
            id,
            principal_id: ic_cdk::caller(),
            farmer_name: new_farmer.farmer_name,
            farm_name: new_farmer.farm_name,
            farm_description: new_farmer.farm_description,
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
            time_for_funding_round_to_expire: None
        };

        let farmer_clone1 = farmer.clone();
        let farmer_clone2 = farmer.clone();

        entitymanagement::FARMER_STORAGE.with(|farmers| {
            farmers.borrow_mut().insert(id, farmer_clone1)
        });

        // Mapping the farmer to the agribusiness 
        entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE.with(|farmers| {
            farmers.borrow_mut().insert(id, farmer_clone2)
        }); 

        Ok(entitymanagement::Success::FarmsAgriBizRegisteredSuccesfully {
            msg: format!("Farm added successfully to the agribusiness: {}", agribusiness_name),
        })

    } else {
        Err(entitymanagement::Error::YouAreNotRegistered {
            msg: "You are not registered as a FarmsAgriBusiness, or the agribusiness name is incorrect.".to_string(),
        })
    }

}


#[query] 
fn get_farms_for_agribusiness() -> Vec<entitymanagement::Farmer> {
   entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE.with(|farms| farms.borrow().iter().map(|(_, item)| item.clone()).collect())
}

#[update] 
fn publish_unpublish(farm_id: u64, publish: bool) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller();

    let mut farms_for_agribusiness = get_farms_for_agribusiness(); 

    if let Some(farm) = farms_for_agribusiness.iter_mut().find(|f| f.principal_id == caller) {
        farm.publish = publish; 

        let farm_clone_1 = farm.clone(); 
        let farm_clone_2 = farm.clone(); 
        
        entitymanagement::FARMS_FOR_AGRIBUSINESS_STORAGE.with(|service| service.borrow_mut().insert(farm_id, farm_clone_1)); 
        
        entitymanagement::FARMER_STORAGE.with(|service| service.borrow_mut().insert(farm_id, farm_clone_2)); 
        
        Ok(entitymanagement::Success::FarmPublishedSuccesfully { 
            msg: format!("Farm publish status succesfully updated to {}", farm.publish) 
        }) 
        
    } else {
        Err(entitymanagement::Error::NotAuthorized { msg: format!("Farm not found!") })
    }
}            

#[update] 
fn delete_farm(farm_id: u64) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller(); 
    
    let mut farms_for_agribusiness = get_farms_for_agribusiness(); 

    if let Some(index) = farms_for_agribusiness.iter().position(|f| f.id == farm_id && f.principal_id == caller) {
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

        Ok(entitymanagement::Success::FarmDeletedSuccesfully { msg: format!("Farm {} has been deleted succesfully", farm.farm_name) }) 
    } else {
        Err(entitymanagement::Error::ErrorOccured { msg: format!("An error occured!") })
    }
}