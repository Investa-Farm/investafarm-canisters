use crate::entitymanagement::{self};
use ic_cdk::{query, update};
use std::time::Duration;

/**
 * Handles the request for a loan by a farmer.
 *
 * @param farm_id The ID of the farm requesting the loan.
 * @param loan_amount The amount of loan requested.
 * @return Result<entitymanagement::Success, entitymanagement::Error> 
 *  Returns a success message if the loan is applied successfully, otherwise returns an error.
 */
#[update]
pub fn ask_for_loan(
    farm_id: u64,
    loan_amount: u64,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    // Retrieve the list of farmers
    let mut farmers = entitymanagement::return_farmers();

    // Find the farmer with the specified farm_id
    if let Some(farm) = farmers.iter_mut().find(|f| f.id == farm_id) {
        // Check if the farm has a credit score
        if let Some(credit_score) = farm.credit_score {
            // Ensure the loan amount is not greater than the credit score
            if loan_amount > credit_score {
                return Err(entitymanagement::Error::Error {
                    msg: format!("Loan ask should not be greater than credit score!"),
                });
            // Ensure the farm is not already processing another loan
            } else if farm.loaned {
                return Err(entitymanagement::Error::Error {
                    msg: format!("You cannot ask for a loan while processing another loan!"),
                });
            }
        } else {
            return Err(entitymanagement::Error::Error {
                msg: format!("Credit score is not available!"),
            });
        }

        // Set the current loan ask amount
        farm.current_loan_ask = Some(loan_amount);
        // Set the funding_round_start_time field to the current time
        farm.funding_round_start_time = Some(ic_cdk::api::time());
        // Set the time_for_funding_round_to_expire field to a duration of 1 month
        // farm.time_for_funding_round_to_expire = Some(Duration::from_secs(30 * 24 * 60 * 60));
        farm.time_for_funding_round_to_expire = Some(Duration::from_secs(3 * 60)); //(use this to test)


        // Reset the loan_maturity
        farm.loan_maturity = None;

        // Clone the updated farm and store it in FARMER_STORAGE
        let farm_clone = farm.clone();
        entitymanagement::FARMER_STORAGE
            .with(|service| service.borrow_mut().insert(farm_id, farm_clone));

        Ok(entitymanagement::Success::AppliedForLoanSuccesfully {
            msg: format!(
                "Loan applied successfully for farm_id: {}. Funding round will close in 1 month.",
                farm_id
            ),
        })
    } else {
        Err(entitymanagement::Error::Error {
            msg: format!("An error occured!"),
        })
    }
}

/**
 * Checks if the loan's funding round has expired and sets the loan maturity duration if it has.
 *
 * @param farm_id The ID of the farm to check for loan expiry.
 * @return Result<(), entitymanagement::Error> 
 *  Returns Ok(()) if the funding round has expired and loan maturity is set, otherwise returns an error.
 */
#[update]
pub fn check_funding_round_expiry(farm_id: u64) -> Result<String, entitymanagement::Error> {
    let farmers = entitymanagement::return_farmers();
    if let Some(farm) = farmers.iter().find(|f| f.id == farm_id) {
        if let (Some(start_time), Some(duration)) = (
            farm.funding_round_start_time,
            farm.time_for_funding_round_to_expire,
        ) {
            // Get the current time and calculate the expiry time
            let current_time = ic_cdk::api::time();
            let expiry_time = start_time + duration.as_nanos() as u64;
            // Check if the current time is greater than or equal to the expiry time
            if current_time >= expiry_time {
                return Ok(format!("Funding round for Farm ID {} has expired", farm_id));
            } else {
                return Ok(format!(
                    "Funding round for Farm ID {} has not expired",
                    farm_id
                ));
            }
        }
    }
    Err(entitymanagement::Error::Error {
        msg: format!("Farm not found or funding round not active!"),
    })
}


#[update]
pub fn initiate_loan(farm_id: u64) -> Result<(), entitymanagement::Error> {
    entitymanagement::FARMER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut farm) = storage.get(&farm_id) {
            let current_time = ic_cdk::api::time();
            farm.funding_round_start_time = None;
            farm.time_for_funding_round_to_expire = None;
            farm.loan_start_time = Some(current_time);
            // farm.loan_maturity = Some(Duration::from_secs(6 * 30 * 24 * 60 * 60));
            farm.loan_maturity = Some(Duration::from_secs(3 * 60)); // 3 minutes (for testing)
            farm.loaned = true;
            storage.insert(farm_id, farm);
            Ok(())
        } else {
            Err(entitymanagement::Error::Error {
                msg: format!("Farm not found!"),
            })
        }
    })
}

/**
 * Gets the remaining time for the funding round.
 *
 * @param farm_id The ID of the farm to get the remaining funding time for.
 * @return Result<u64, entitymanagement::Error> 
 *  Returns the remaining funding time in seconds if an active funding round is found, otherwise returns an error.
 */
#[query]
pub fn get_remaining_funding_time(farm_id: u64) -> Result<u64, entitymanagement::Error> {
    // Retrieve the list of farmers
    let farmers = entitymanagement::return_farmers();

    // Find the farmer with the specified farm_id
    if let Some(farm) = farmers.iter().find(|f| f.id == farm_id) {
        // Check if the funding round start time and duration are available
        if let (Some(start_time), Some(duration)) = (
            farm.funding_round_start_time,
            farm.time_for_funding_round_to_expire,
        ) {
            // Get the current time and calculate the expiry time
            let current_time = ic_cdk::api::time();
            let expiry_time = start_time + duration.as_nanos() as u64;
            // Check if the current time is less than the expiry time
            if current_time < expiry_time {
                // Return the remaining funding time in seconds
                return Ok((expiry_time - current_time) / 1_000_000_000); // Convert nanoseconds to seconds
            }
        }
    }
    Err(entitymanagement::Error::Error {
        msg: format!("No active funding round or farm not found!"),
    })
}

#[query]
pub fn get_remaining_loan_maturity_time(farm_id: u64) -> Result<u64, entitymanagement::Error> {
    let farmers = entitymanagement::return_farmers();
    if let Some(farm) = farmers.iter().find(|f| f.id == farm_id) {
        if let (Some(loan_start_time), Some(loan_duration)) =
            (farm.loan_start_time, farm.loan_maturity)
        {
            let current_time = ic_cdk::api::time();
            let elapsed = current_time.saturating_sub(loan_start_time);
            let total_duration = loan_duration.as_nanos() as u64;
            let remaining = total_duration.saturating_sub(elapsed);
            return Ok(remaining / 1_000_000_000); // Convert nanoseconds to seconds
        }
    }
    Err(entitymanagement::Error::Error {
        msg: format!("Loan not found or not active for farm_id: {}", farm_id),
    })
}
