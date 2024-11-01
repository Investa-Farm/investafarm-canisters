use std::{cell::RefCell, collections::HashMap};
use candid::Principal;
use ic_cdk::{
    query, update
};

thread_local! {
    static APPROVED_SPENDERS: RefCell<HashMap<Principal, Vec<Principal>>> = RefCell::new(HashMap::new());
}

// Function to check if spender is already approved
#[query]
fn is_spender_approved(owner: Principal, spender: Principal) -> bool {
    APPROVED_SPENDERS.with(|approved| {
        approved.borrow()
            .get(&owner)
            .map_or(false, |spenders| spenders.contains(&spender))
    })
}

// Function to store approved spender
#[update]
fn store_approved_spender(owner: Principal, spender: Principal) -> Result<(), String> {
    APPROVED_SPENDERS.with(|approved| {
        let mut approved = approved.borrow_mut();
        approved.entry(owner)
            .or_insert_with(Vec::new)
            .push(spender);
        Ok(())
    })
}
