use std::collections::HashMap;
use std::cell::RefCell;

use ic_cdk::{query, update}; 

// Defining structs 
struct InvestorInvestments {
    investments: HashMap<u64, Vec<(u64, f64, String)>>, // investor_id => [(farm_id, amount), ...]
} 

struct FarmInvestments {
    investments: HashMap<u64, Vec<(u64, f64, String)>>, // farm_id => [(investor_id, amount), ...]
}

thread_local! {
    static INVESTOR_INVESTMENTS: RefCell<InvestorInvestments> = RefCell::new(InvestorInvestments { investments: HashMap::new() });
    static FARM_INVESTMENTS: RefCell<FarmInvestments> = RefCell::new(FarmInvestments { investments: HashMap::new() });
}

#[update] 
fn store_investments(
    farm_id: u64, 
    amount: f64, 
    investor_id: u64, 
    transaction_hash: String
) -> Result<(), String> {
    INVESTOR_INVESTMENTS.with(|investor_investments| {
        let mut investor_investments = investor_investments.borrow_mut();
        investor_investments.investments.entry(investor_id).or_insert_with(|| Vec::new()).push((farm_id, amount, transaction_hash.clone())); 
    });

    FARM_INVESTMENTS.with(|farm_investments| {
        let mut farm_investments = farm_investments.borrow_mut();
        farm_investments.investments.entry(farm_id).or_insert_with(|| Vec::new()).push((investor_id, amount, transaction_hash));
    });

    Ok(())
}

#[query]
fn get_investments_by_investor(investor_id: u64) -> Option<Vec<(u64, f64, String)>> {
    INVESTOR_INVESTMENTS.with(|investor_investments| {
        let investor_investments = investor_investments.borrow();
        investor_investments.investments.get(&investor_id).cloned()
    })
}

#[query]
fn get_investments_by_farm(farm_id: u64) -> Option<Vec<(u64, f64, String)>> {
    FARM_INVESTMENTS.with(|farm_investments| {
        let farm_investments = farm_investments.borrow();
        farm_investments.investments.get(&farm_id).cloned()
    })
}