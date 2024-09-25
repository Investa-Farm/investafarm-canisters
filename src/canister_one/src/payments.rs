use std::collections::HashMap;
use std::cell::RefCell;

use candid::CandidType;
use ic_cdk::{query, update};
use serde::Deserialize;

// Defining structs 
#[derive(Clone, CandidType, Deserialize)]
pub struct InvestorInvestments {
    investments: HashMap<u64, Vec<(u64, f64, String, String)>>, // investor_id => [(farm_id, amount, transaction_hash, currency), ...]
}

#[derive(Clone, CandidType, Deserialize)]
pub struct FarmInvestments {
    investments: HashMap<u64, Vec<(u64, f64, String, String)>>, // farm_id => [(investor_id, amount), ...]
}

impl InvestorInvestments {
    pub fn clone_investments(&self) -> Self {
        InvestorInvestments {
            investments: self.investments.clone(),
        }
    }
}

impl FarmInvestments {
    pub fn clone_investments(&self) -> Self {
        FarmInvestments {
            investments: self.investments.clone(),
        }
    }
}

thread_local! {
    pub static INVESTOR_INVESTMENTS: RefCell<InvestorInvestments> = RefCell::new(InvestorInvestments { investments: HashMap::new() });
    pub static FARM_INVESTMENTS: RefCell<FarmInvestments> = RefCell::new(FarmInvestments { investments: HashMap::new() });
}

#[update] 
pub fn store_investments(
    farm_id: u64, 
    amount: f64, 
    investor_id: u64, 
    transaction_hash: String, 
    currency: String
) -> Result<(), String> {
    INVESTOR_INVESTMENTS.with(|investor_investments| {
        let mut investor_investments = investor_investments.borrow_mut();
        investor_investments.investments.entry(investor_id).or_insert_with(|| Vec::new()).push((farm_id, amount, transaction_hash.clone(), currency.clone())); 
    });

    FARM_INVESTMENTS.with(|farm_investments| {
        let mut farm_investments = farm_investments.borrow_mut();
        farm_investments.investments.entry(farm_id).or_insert_with(|| Vec::new()).push((investor_id, amount, transaction_hash, currency.clone()));
    });

    Ok(())
}

#[query]
fn get_investments_by_investor(investor_id: u64) -> Option<Vec<(u64, f64, String, String)>> {
    INVESTOR_INVESTMENTS.with(|investor_investments| {
        let investor_investments = investor_investments.borrow();
        investor_investments.investments.get(&investor_id).cloned()
    })
}

#[query]
fn get_investments_by_farm(farm_id: u64) -> Option<Vec<(u64, f64, String, String)>> {
    FARM_INVESTMENTS.with(|farm_investments| {
        let farm_investments = farm_investments.borrow();
        farm_investments.investments.get(&farm_id).cloned()
    })
}

// Calculating total investments recieved by a farm
#[query]
fn calculate_total_investments_received_by_farm(farm_id: u64) -> f64 {
    FARM_INVESTMENTS.with(|farm_investments| {
        let farm_investments = farm_investments.borrow();
        farm_investments.investments.get(&farm_id)
            .map(|investments| investments.iter().map(|(_, amount, _, _)| amount).sum())
            .unwrap_or(0.0)
    })
}

// Calculating total investments made by an investor on a specific farm 
#[query]    
fn calculate_total_investments_by_investor_on_farm(investor_id: u64, farm_id: u64) -> f64 {
    INVESTOR_INVESTMENTS.with(|investor_investments| {
        let investor_investments = investor_investments.borrow();
        investor_investments.investments.get(&investor_id)
            .map(|investments| {
                investments.iter()
                    .filter(|(f_id, _, _, _)| *f_id == farm_id)
                    .map(|(_, amount, _, _)| amount)
                    .sum()
            })
            .unwrap_or(0.0)
    })
}

// Calculating total investments made by an investor across all farms
#[query]
fn calculate_total_investments_by_investor(investor_id: u64) -> f64 {
    INVESTOR_INVESTMENTS.with(|investor_investments| {
        let investor_investments = investor_investments.borrow();
        investor_investments.investments.get(&investor_id)
            .map(|investments| investments.iter().map(|(_, amount, _, _)| amount).sum())
            .unwrap_or(0.0)
    })
}