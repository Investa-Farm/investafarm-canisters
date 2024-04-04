use ic_cdk::update; 

mod entitymanagement;

// REGISTER FARMS FUNCTION 
#[update]
fn register_your_farm(new_farmer: entitymanagement::NewFarmer) ->  Result<entitymanagement::Success, entitymanagement::Error> {
    entitymanagement::register_farm(new_farmer)
}

// REGISTER INVESTOR FUNCTION 
#[update] 
fn register_investor(new_investor: entitymanagement::NewInvestor) -> Result<entitymanagement::Success, entitymanagement::Error> {
    entitymanagement::register_investor(new_investor)
}

ic_cdk::export_candid!(); 