use ic_cdk::update; 

mod entitymanagement;

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

ic_cdk::export_candid!(); 