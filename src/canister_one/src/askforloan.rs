use crate::entitymanagement::{self};
use ic_cdk::{query, update};
use std::time::Duration;

#[update]
pub fn ask_for_loan(
    farm_id: u64,
    loan_amount: u64,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let mut farmers = entitymanagement::return_farmers();

    if let Some(farm) = farmers.iter_mut().find(|f| f.id == farm_id) {
        if let Some(credit_score) = farm.credit_score {
            if loan_amount > credit_score {
                return Err(entitymanagement::Error::Error {
                    msg: format!("Loan ask should not be greater than credit score!"),
                });
            } else if farm.loaned == true {
                return Err(entitymanagement::Error::Error {
                    msg: format!("You cannot ask for a loan while processing another loan!"),
                });
            }
        } else {
            return Err(entitymanagement::Error::Error {
                msg: format!("Credit score is not available!"),
            });
        }

        farm.current_loan_ask = Some(loan_amount);
        // Set the funding_round_start_time field to the current time
        farm.funding_round_start_time = Some(ic_cdk::api::time());
        // Set the time_for_funding_round_to_expire field to a duration of 1 month
        // farm.time_for_funding_round_to_expire = Some(Duration::from_secs(30 * 24 * 60 * 60));
        farm.time_for_funding_round_to_expire = Some(Duration::from_secs(3 * 60)); //(use this to test)

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

#[update]
pub fn check_funding_round_expiry(farm_id: u64) -> Result<bool, entitymanagement::Error> {
    let farmers = entitymanagement::return_farmers();
    if let Some(farm) = farmers.iter().find(|f| f.id == farm_id) {
        if let (Some(start_time), Some(duration)) = (
            farm.funding_round_start_time,
            farm.time_for_funding_round_to_expire,
        ) {
            let current_time = ic_cdk::api::time();
            let expiry_time = start_time + duration.as_nanos() as u64;
            return Ok(current_time >= expiry_time);
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

#[query]
pub fn get_remaining_funding_time(farm_id: u64) -> Result<u64, entitymanagement::Error> {
    let farmers = entitymanagement::return_farmers();
    if let Some(farm) = farmers.iter().find(|f| f.id == farm_id) {
        if let (Some(start_time), Some(duration)) = (
            farm.funding_round_start_time,
            farm.time_for_funding_round_to_expire,
        ) {
            let current_time = ic_cdk::api::time();
            let expiry_time = start_time + duration.as_nanos() as u64;
            if current_time < expiry_time {
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
