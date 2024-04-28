use ic_cdk::update;

use crate::entitymanagement;

#[update]
pub fn add_farm_to_agribusiness(agribusiness_name: String, farm_id: u64) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let caller = ic_cdk::caller();
    let mut agribusiness_found = false;

    entitymanagement::REGISTERED_FARMS_AGRIBUSINESS.with(|agribusinesses| {
        if let Some(agribusiness) = agribusinesses.borrow().get(&agribusiness_name) {
            if agribusiness.principal_id == caller {
                let mut farms = agribusiness.farms.clone().unwrap_or_default();
                let farmer = entitymanagement::FARMER_STORAGE.with(|farmers| farmers.borrow().get(&farm_id));

                if let Some(farmer) = farmer {
                    farms.insert(farmer.clone(), 0); // Initialize the value to 0, you can update it later if needed
                    
                    let updated_agribusiness = entitymanagement::FarmsAgriBusiness {
                        id: agribusiness.id,
                        agribusiness_name: agribusiness.agribusiness_name.clone(),
                        total_farmers: agribusiness.total_farmers + 1,
                        principal_id: agribusiness.principal_id,
                        verified: agribusiness.verified,
                        farms: Some(farms),
                    };

                    entitymanagement::FARMS_AGRIBUSINESS_STORAGE.with(|agribusinesses| {
                        agribusinesses.borrow_mut().insert(agribusiness.id, updated_agribusiness);
                    });

                    agribusiness_found = true;
                }
            }
        }
    });

    if agribusiness_found {
        Ok(entitymanagement::Success::FarmsAgriBizRegisteredSuccesfully {
            msg: format!("Farm added successfully to the agribusiness: {}", agribusiness_name),
        })
    } else {
        Err(entitymanagement::Error::YouAreNotRegistered {
            msg: "You are not registered as a FarmsAgriBusiness, or the agribusiness name is incorrect.".to_string(),
        })
    }
}