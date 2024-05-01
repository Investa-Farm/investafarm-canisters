use candid::Principal;
use ic_cdk::{query, update};

use crate::entitymanagement::{self, Farmer};

// #[update]
// fn add_farm_to_agribusiness(agribusiness_name: String, farm_id: u64) -> Result<entitymanagement::Success, entitymanagement::Error> {
// // fn add_farm_to_agribusiness(agribusiness_name: String, farm_id: u64) -> Farmer {
//     let caller = ic_cdk::caller();
//     let mut agribusiness_found = false;

//     entitymanagement::REGISTERED_FARMS_AGRIBUSINESS.with(|agribusinesses| {
//         if let Some(mut agribusiness) = agribusinesses.borrow_mut().get_mut(&agribusiness_name) {
//             if agribusiness.principal_id == caller {
//                 let farmer  = entitymanagement::FARMER_STORAGE.with(|farmers| farmers.borrow().get(&farm_id)); 
                 
//                 if let Some(farmer) = farmer {
//                     let mut farms = agribusiness.farms.get_or_insert_with(entitymanagement::FarmsForAgriBusiness::new); 
                    
//                     farms.insert(farmer.clone(), 0); 
//                     agribusiness.total_farmers += 1; 

//                     // let updated_agribusiness = entitymanagement::FarmsAgriBusiness {
//                     //     id: agribusiness.id,
//                     //     agribusiness_name: agribusiness.agribusiness_name.clone(),
//                     //     total_farmers: agribusiness.total_farmers + 1,
//                     //     principal_id: agribusiness.principal_id,
//                     //     verified: agribusiness.verified,
//                     //     farms: Some(farms),
//                     // };

//                     // entitymanagement::FARMS_AGRIBUSINESS_STORAGE.with(|agribusinesses| {
//                     //     agribusinesses.borrow_mut().insert(agribusiness.id, updated_agribusiness);
//                     // });

//                     agribusiness_found = true;
//                 }
//             }
//         }
//     });

//     if agribusiness_found {
//         Ok(entitymanagement::Success::FarmsAgriBizRegisteredSuccesfully {
//             msg: format!("Farm added successfully to the agribusiness: {}", agribusiness_name),
//         })
//     } else {
//         Err(entitymanagement::Error::YouAreNotRegistered {
//             msg: "You are not registered as a FarmsAgriBusiness, or the agribusiness name is incorrect.".to_string(),
//         })
//     }
// }

#[update] 
fn add_farm_to_agribusiness(new_farmer: entitymanagement::NewFarmer, agribusiness_name: String, agribusiness_id: u64) -> Result<entitymanagement::Success, entitymanagement::Error> {
    if new_farmer.farmer_name.is_empty() || new_farmer.farm_name.is_empty() || new_farmer.farm_description.is_empty() {
        return Err(entitymanagement::Error::FieldEmpty { msg: format!("Kindly ensure all required fieilds are filled!") })
    }

    let id = entitymanagement::FARMER_ID.with(|id| entitymanagement::_increament_id(id)); 
     
    let response = entitymanagement::_is_principal_id_registered(ic_cdk::caller()); 

    if let Err(entitymanagement::Error::PrincipalIdAlreadyRegistered { msg }) = response {
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
            insured: None
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
