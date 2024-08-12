use std::collections::HashMap;
use std::cell::RefCell;

thread_local! {
    pub static TRANSACTION_FEES: RefCell<HashMap<String, f64>> = RefCell::new(HashMap::new());
}

#[ic_cdk::update]
pub fn store_transaction_fee(transaction_hash: String, fee: f64) -> Result<(), String> {
    TRANSACTION_FEES.with(|transaction_fees| {
        let mut transaction_fees = transaction_fees.borrow_mut();
        transaction_fees.insert(transaction_hash, fee);
    });
    Ok(())
}

#[ic_cdk::query]
pub fn get_all_transaction_fees() -> Vec<(String, f64)> {
    TRANSACTION_FEES.with(|transaction_fees| {
        let transaction_fees = transaction_fees.borrow();
        transaction_fees.iter().map(|(k, v)| (k.clone(), *v)).collect()
    })
}