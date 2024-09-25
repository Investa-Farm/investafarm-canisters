use std::collections::HashMap;

use crate::entitymanagement::{Error, Success};
use crate::farmerfiles::FarmerReport;
use crate::entitymanagement::EntityType; 
use crate::entitymanagement::EntityDetails; 
use crate::entitymanagement::Farmer;  
use crate::entitymanagement::Investor;  
use crate::entitymanagement::FarmsAgriBusiness; 
use crate::farmsagribizmanagement::RegisterFarm; 
use crate::ck_eth::receipt; 
use crate::ck_eth::minter; 
use crate::entitymanagement::FinancialReport;
use crate::entitymanagement::FarmReport;
// use crate::askforloan;
use ic_cdk::{query, update};
use candid::Principal;
use candid::Nat;

use b3_utils::ledger::ICRC1TransferResult;

mod adminapproval;
mod askforloan;
mod creditscore;
mod entitymanagement;
mod farmerfiles;
mod farmsagribizmanagement;
mod payments;
mod common;
mod ck_eth;
mod ck_eth_payments;
mod transaction_fees;
// mod supplymanagement;
mod ckusdc_payments;

use ic_cdk::storage;

// REGISTER FARMS
#[update]
fn register_your_farm(
    new_farmer: entitymanagement::NewFarmer,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    entitymanagement::register_farm(new_farmer)
}

// REGISTER INVESTOR
#[update]
fn register_investor(
    new_investor: entitymanagement::NewInvestor,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    entitymanagement::register_investor(new_investor)
}

// REGISTER SUPPLY AGRIBUSINESS
#[update]
fn register_supply_agribusiness(
    new_supply_agribusiness: entitymanagement::NewSupplyAgriBusiness,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
    entitymanagement::register_supply_agribusiness(new_supply_agribusiness)
}

// REGISTER FARMS AGRI BUSINESS
#[update]
fn register_farms_agribusiness(
    new_farms_agribusiness: entitymanagement::NewFarmsAgriBusiness,
) -> Result<entitymanagement::Success, entitymanagement::Error> {
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

// Saving Stable State
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let investor_investments = payments::INVESTOR_INVESTMENTS.with(|investor_investments| investor_investments.borrow().clone_investments());
    let farm_investments = payments::FARM_INVESTMENTS.with(|farm_investments| farm_investments.borrow().clone_investments());
    let transaction_fees = transaction_fees::TRANSACTION_FEES.with(|transaction_fees| transaction_fees.borrow().clone());
    
    storage::stable_save((
        investor_investments,
        farm_investments,
        transaction_fees
    )).expect("Failed to save stable state");
}

// Restoring Stable State
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let (investor_investments, farm_investments, transaction_fees): (payments::InvestorInvestments, payments::FarmInvestments, HashMap<String, f64>) =
        storage::stable_restore().expect("Failed to restore stable state");

    payments::INVESTOR_INVESTMENTS.with(|investor_investments_cell| {
        *investor_investments_cell.borrow_mut() = investor_investments;
    });

    payments::FARM_INVESTMENTS.with(|farm_investments_cell| {
        *farm_investments_cell.borrow_mut() = farm_investments;
    });

    transaction_fees::TRANSACTION_FEES.with(|transaction_fees_cell| {
        *transaction_fees_cell.borrow_mut() = transaction_fees;
    });
}

ic_cdk::export_candid!();