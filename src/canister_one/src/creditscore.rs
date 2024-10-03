// use ic_cdk::call;
use ic_cdk::update; 
use candid::Nat;
use crate::entitymanagement::{self};

use crate::ifarm_tokens;

// use b3_utils::http::{HttpRequest, HttpResponse};
use b3_utils::outcall::{HttpOutcall, HttpOutcallResponse}; 
use serde_json::Value;
use std::collections::HashMap;
use num_bigint::BigUint;
// use serde_bytes;

#[update]
async fn fetch_credit_score(mpesa_statement: Vec<u8>, passcode: String) -> Result<u64, String> {
    let url = "http://217.76.59.68:4000/parse_pdf";
    let headers = vec![
        ("Content-Type".to_string(), "multipart/form-data".to_string()),
    ];

    let mut form_data = HashMap::new();
    form_data.insert("file", mpesa_statement);
    form_data.insert("password", passcode.into_bytes());

    // Create an HttpOutcall with the URL
    let outcall = HttpOutcall::new(url)
    .post(&serde_json::to_string(&form_data).unwrap(), Some(2 * 1024 * 1024)) // Set the body and max response bytes
    .add_headers(headers);

    // Send the request
    let response: HttpOutcallResponse = outcall.send().await.map_err(|e| format!("HTTP request failed: {:?}", e))?;

    if response.status.0 == BigUint::from(200u64) {
        let response_body: Value = serde_json::from_slice(&response.body)
            .map_err(|e| format!("Failed to parse response: {:?}", e))?;
        
        if let Some(credit_score) = response_body.get("credit_score").and_then(|v| v.as_u64()) {
            Ok(credit_score)
        } else {
            Err("Credit score not found in response".to_string())
        }
    } else {
        Err(format!("HTTP request failed with status: {}", response.status))
    }
}

#[update]
async fn add_credit_score(farm_id: u64, credit_score: u64) -> Result<entitymanagement::Success, entitymanagement::Error> {

    let mut farmers = entitymanagement::return_farmers(); 

    // Transfer ifarm token to the caller
    let caller = ic_cdk::caller(); 
    let amount = Nat::from(credit_score); 
    let _ = ifarm_tokens::ifarm_transfer(caller, amount).await; 


    if let Some(farm) = farmers.iter_mut().find(|f| f.id == farm_id) {
        farm.credit_score = Some(credit_score); 

        let farm_clone = farm.clone(); 
        entitymanagement::FARMER_STORAGE.with(|service| service.borrow_mut().insert(farm_id, farm_clone)); 

        Ok(entitymanagement::Success::CreditScoreAdded { 
            msg: format!("Credit score updated for farm_id: {}", farm_id)
        })
    } else {
        Err(entitymanagement::Error::Error { msg: format!("An error occured!") })
    }
    
}