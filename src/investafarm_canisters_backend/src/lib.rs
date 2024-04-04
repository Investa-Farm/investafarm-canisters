use ic_cdk::{query, update}; 

mod entitymanagement;

#[update]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// Checking whether you can call user management
#[query] 
fn checking_user_management() -> String {
    entitymanagement::check_user_management()
}

// REGISTER FARMS FUNCTION 
#[update]
fn register_your_farm(new_farmer: entitymanagement::NewFarmer) ->  Result<entitymanagement::Success, entitymanagement::Error> {
    entitymanagement::register_farm(new_farmer)
}

ic_cdk::export_candid!(); 