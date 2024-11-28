use std::collections::HashMap;
use std::cell::RefCell;

use candid::{CandidType, Decode, Encode};
use ic_cdk::{query, update};
use serde::Deserialize;
use crate::entitymanagement::{Memory, MEMORY_MANAGER}; 

use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{StableBTreeMap, BoundedStorable, Storable};
use std::borrow::Cow;

// Defining structs 
#[derive(Clone, CandidType, Deserialize)]
pub struct InvestorInvestments {
    investments: HashMap<u64, Vec<(u64, f64, String, String)>>, // investor_id => [(farm_id, amount, transaction_hash, currency), ...]
}

#[derive(Clone, CandidType, Deserialize)]
pub struct FarmInvestments {
    investments: HashMap<u64, Vec<(u64, f64, String, String)>>, // farm_id => [(investor_id, amount), ...]
}

impl Storable for InvestorInvestments {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for InvestorInvestments {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for FarmInvestments {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FarmInvestments {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    pub static INVESTOR_INVESTMENTS: RefCell<StableBTreeMap<u64, InvestorInvestments, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
        ));

    pub static FARM_INVESTMENTS: RefCell<StableBTreeMap<u64, FarmInvestments, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
        ));
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
        let mut storage = investor_investments.borrow_mut();
        let mut investments = storage.get(&investor_id)
            .unwrap_or_else(|| InvestorInvestments { investments: HashMap::new() });
        
        investments.investments.entry(investor_id)
            .or_insert_with(Vec::new)
            .push((farm_id, amount, transaction_hash.clone(), currency.clone()));
        
        storage.insert(investor_id, investments);
    });

    FARM_INVESTMENTS.with(|farm_investments| {
        let mut storage = farm_investments.borrow_mut();
        let mut investments = storage.get(&farm_id)
            .unwrap_or_else(|| FarmInvestments { investments: HashMap::new() });
        
        investments.investments.entry(farm_id)
            .or_insert_with(Vec::new)
            .push((investor_id, amount, transaction_hash, currency));
        
        storage.insert(farm_id, investments);
    });

    Ok(())
}

#[query]
fn get_investments_by_investor(investor_id: u64) -> Option<Vec<(u64, f64, String, String)>> {
    INVESTOR_INVESTMENTS.with(|storage| {
        storage.borrow()
            .get(&investor_id)
            .map(|investments| investments.investments.get(&investor_id).cloned())
            .flatten()
    })
}

#[query]
fn get_investments_by_farm(farm_id: u64) -> Option<Vec<(u64, f64, String, String)>> {
    FARM_INVESTMENTS.with(|storage| {
        storage.borrow()
            .get(&farm_id)
            .map(|investments| investments.investments.get(&farm_id).cloned())
            .flatten()
    })
}

// Calculating total investments recieved by a farm
#[query]
fn calculate_total_investments_received_by_farm(farm_id: u64) -> f64 {
    FARM_INVESTMENTS.with(|storage| {
        storage.borrow()
            .get(&farm_id)
            .map(|investments| {
                investments.investments.get(&farm_id)
                    .map(|invs| invs.iter().map(|(_, amount, _, _)| amount).sum())
                    .unwrap_or(0.0)
            })
            .unwrap_or(0.0)
    })
}

// Calculating total investments made by an investor on a specific farm 
#[query]    
fn calculate_total_investments_by_investor_on_farm(investor_id: u64, farm_id: u64) -> f64 {
    INVESTOR_INVESTMENTS.with(|storage| {
        storage.borrow()
            .get(&investor_id)
            .map(|investments| {
                investments.investments.get(&investor_id)
                    .map(|invs| {
                        invs.iter()
                            .filter(|(f_id, _, _, _)| *f_id == farm_id)
                            .map(|(_, amount, _, _)| amount)
                            .sum()
                    })
                    .unwrap_or(0.0)
            })
            .unwrap_or(0.0)
    })
}

// Calculating total investments made by an investor across all farms
#[query]
fn calculate_total_investments_by_investor(investor_id: u64) -> f64 {
    INVESTOR_INVESTMENTS.with(|storage| {
        storage.borrow()
            .get(&investor_id)
            .map(|investments| {
                investments.investments.get(&investor_id)
                    .map(|invs| invs.iter().map(|(_, amount, _, _)| amount).sum())
                    .unwrap_or(0.0)
            })
            .unwrap_or(0.0)
    })
}