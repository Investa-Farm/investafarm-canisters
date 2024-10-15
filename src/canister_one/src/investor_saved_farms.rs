use ic_cdk::query;
use ic_cdk::update;
use ic_cdk::caller;

// Assuming these are already defined in your code
use crate::entitymanagement::{ INVESTOR_STORAGE, check_entity_type, EntityType, display_specific_farm };
use crate::Error;
use crate::Success;

/**
* Function: update_investor_saved_farms
* Description: Adds a single farm ID to the saved farms for a specific investor.
* @param farm_id: u64 - The farm ID to be added to saved farms
* @return Result<Success, Error> - Success message if update is successful, or an error message otherwise
*/
#[update]
pub fn update_investor_saved_farms(farm_id: u64) -> Result<Success, Error> {
    // Check if the caller is an investor
    if check_entity_type() != EntityType::Investor {
        return Err(Error::Unauthorized {
            msg: format!("Only investors can save farms"),
        });
    }

    // Check if the farm exists
    if display_specific_farm(farm_id).is_err() {
        return Err(Error::NotFound {
            msg: format!("Farm with ID {} not found", farm_id),
        });
    }

    let caller_id = caller();

    INVESTOR_STORAGE.with(|investors| {
        let mut investors = investors.borrow_mut();
        
        // Find the investor by principal ID
        let investor_id = investors
            .iter()
            .find(|(_, investor)| investor.principal_id == caller_id)
            .map(|(id, _)| id) 
            .ok_or(Error::NotFound {
                msg: format!("Investor not found for the caller"),
            })?;

        if let Some(mut investor) = investors.remove(&investor_id) {
            let mut saved_farms = investor.saved_farms.clone().unwrap_or_else(Vec::new);
            if !saved_farms.contains(&farm_id) {
                saved_farms.push(farm_id);
                investor.saved_farms = Some(saved_farms);
                investors.insert(investor_id, investor);
                Ok(Success::InvestorUpdatedSuccessfully {
                    msg: format!("Farm {} added to saved farms for investor {}", farm_id, investor_id),
                })
            } else {
                investors.insert(investor_id, investor);
                Ok(Success::InvestorUpdatedSuccessfully {
                    msg: format!("Farm {} was already in saved farms for investor {}", farm_id, investor_id),
                })
            }
        } else {
            Err(Error::NotFound {
                msg: format!("Investor data not found for the caller"),
            })
        }
    })
}

/**
* Function: get_investor_saved_farms
* Description: Retrieves the saved farms for a specific investor.
* @param investor_id: u64 - The unique identifier of the investor
* @return Result<Vec<u64>, Error> - A vector of saved farm IDs if successful, or an error message otherwise
*/
#[query]
pub fn get_investor_saved_farms(investor_id: u64) -> Result<Vec<u64>, Error> {
    INVESTOR_STORAGE.with(|investors| {
        let investors = investors.borrow();
        
        if let Some(investor) = investors.get(&investor_id) {
            investor.saved_farms.clone().ok_or(Error::NotFound {
                msg: format!("No saved farms found for investor {}", investor_id),
            })
        } else {
            Err(Error::NotFound {
                msg: format!("Investor with ID {} not found", investor_id),
            })
        }
    })
}