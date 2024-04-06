#[ic_cdk::query]
fn call() -> String {
   return "This is the 3rd canister".to_string()
}

ic_cdk::export_candid!();  