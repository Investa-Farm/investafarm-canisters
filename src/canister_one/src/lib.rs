use ic_cdk::{query, update}; 
use crate::entitymanagement::Error; 
use ic_ledger_types::BlockIndex; 
use crate::payments::TransferArgs;

mod entitymanagement;
mod adminapproval;
mod payments; 

// REGISTER FARMS 
#[update]
fn register_your_farm(new_farmer: entitymanagement::NewFarmer) ->  Result<entitymanagement::Success, entitymanagement::Error> {
    entitymanagement::register_farm(new_farmer)
}

// REGISTER INVESTOR 
#[update] 
fn register_investor(new_investor: entitymanagement::NewInvestor) -> Result<entitymanagement::Success, entitymanagement::Error> {
    entitymanagement::register_investor(new_investor)
}

// REGISTER SUPPLY AGRIBUSINESS 
#[update]
fn register_supply_agribusiness(new_supply_agribusiness: entitymanagement::NewSupplyAgriBusiness) -> Result<entitymanagement::Success, entitymanagement::Error> {
    entitymanagement::register_supply_agribusiness(new_supply_agribusiness)
}

// REGISTER FARMS AGRI BUSINESS 
#[update] 
fn register_farms_agribusiness(new_farms_agribusiness: entitymanagement::NewFarmsAgriBusiness) -> Result<entitymanagement::Success, entitymanagement::Error> {
    entitymanagement::register_farms_agribusiness(new_farms_agribusiness)
}

// DISPLAY FARMS THAT HAVE REGISTERED 
#[query]
fn display_farms() -> Vec<entitymanagement::Farmer> {
    entitymanagement::return_farmers()
}

// DISPLAY INVESTORS THAT HAVE REGISTERED 
#[query]
fn display_investors() -> Vec<entitymanagement::Investor> {
    entitymanagement::return_investors()
}

// DISPLAY SUPPLY AGRI BIZ THAT HAVE REGISTERED 
#[query] 
fn display_supply_agribusinesses() -> Vec<entitymanagement::SupplyAgriBusiness> {
    entitymanagement::return_supply_agribusiness()
}

// DISPLAY FARMS AGRI BIZ THAT HAVE REGISTERED 
#[query] 
fn display_farms_agribusinesses() -> Vec<entitymanagement::FarmsAgriBusiness> {
    entitymanagement::return_farms_agribusiness()
}

// fn display_test_ledger() -> Result<String, payments::Error> {
//     payments::test_ledger()
// }

ic_cdk::export_candid!(); 