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
use ic_cdk::println; 
// use crate::adminapproval::is_allowed_principal;  
// use serde_bytes;

// const CHUNK_SIZE: usize = 2_000_000; // 2MB 

// #[update]
// async fn fetch_credit_score(mpesa_statement: Vec<u8>, passcode: String) -> Result<u64, String> {
//     let url = "http://217.76.59.68:4000/parse_pdf";
//     let headers = vec![
//         ("Content-Type".to_string(), "multipart/form-data".to_string()),
//     ];

//     let mut form_data = HashMap::new();
//     form_data.insert("file", mpesa_statement);
//     form_data.insert("password", passcode.into_bytes());
//     println!("Form data: {:?}", form_data);

//      // Create an HttpOutcall with the URL
//      let outcall = HttpOutcall::new(url)
//      .post(&serde_json::to_string(&form_data).unwrap(), Some(2_000_000)) // Set the body and max response bytes
//      .add_headers(headers);

//     // Send the request
//     let response: HttpOutcallResponse = outcall.send().await.map_err(|e| format!("HTTP request failed: {:?}", e))?;

//     if response.status.0 == BigUint::from(200u64) {
//         let response_body: Value = serde_json::from_slice(&response.body)
//             .map_err(|e| format!("Failed to parse response: {:?}", e))?;
        
//         if let Some(credit_score) = response_body.get("credit_score").and_then(|v| v.as_u64()) {
//             Ok(credit_score)
//         } else {
//             Err("Credit score not found in response".to_string())
//         }
//     } else {
//         Err(format!("HTTP request failed with status: {}", response.status))
//     }
// }

// TRYING USING CHUNKED UPLOAD 
const CHUNK_SIZE: usize = 2_000_000; // 2MB

#[update]
async fn fetch_credit_score(mpesa_statement: Vec<u8>, passcode: String) -> Result<u64, String> {
    let url = "http://192.168.100.169:4000/parse_pdf"; // Locally 
    // let url = "http://217.76.59.68:4000/parse_pdf"; // On server 
    let headers = vec![
        ("Content-Type".to_string(), "multipart/form-data".to_string()),
    ];

    // Split the data into chunks
    let chunks: Vec<&[u8]> = mpesa_statement.chunks(CHUNK_SIZE).collect();
    // println!("Chunks: {:?}", chunks);
    let total_chunks = chunks.len();
    // println!("Total chunks: {:?}", total_chunks);

    for (index, chunk) in chunks.iter().enumerate() {
        let mut form_data = HashMap::new();
        form_data.insert("file", chunk.to_vec());
        form_data.insert("password", passcode.clone().into_bytes());
        form_data.insert("chunk_index", index.to_string().into_bytes());
        form_data.insert("total_chunks", total_chunks.to_string().into_bytes());

        println!("Form data: {:?}", form_data);

        // Create an HttpOutcall with the URL
        let outcall = HttpOutcall::new(url)
            .post(&serde_json::to_string(&form_data).unwrap(), Some(2_000_000)) // Set the body and max response bytes
            .add_headers(headers.clone());


        // Send the request
        let response: HttpOutcallResponse = outcall.send().await.map_err(|e| format!("HTTP request failed: {:?}", e))?;

        println!("Response: {:?}", response);

        if response.status.0 != BigUint::from(200u64) {
            return Err(format!("HTTP request failed with status: {}", response.status));
        }
    }

    // Assuming the server sends back the credit score after all chunks are received
    // You might need to adjust this part based on your server's response handling
    let final_response: HttpOutcallResponse = HttpOutcall::new(url)
        .get(Some(2_000_000))
        .send()
        .await
        .map_err(|e| format!("Final HTTP request failed: {:?}", e))?;

    if final_response.status.0 == BigUint::from(200u64) {
        let response_body: Value = serde_json::from_slice(&final_response.body)
            .map_err(|e| format!("Failed to parse response: {:?}", e))?;
        
        if let Some(credit_score) = response_body.get("credit_score").and_then(|v| v.as_u64()) {
            Ok(credit_score)
        } else {
            Err("Credit score not found in response".to_string())
        }
    } else {
        Err(format!("Final HTTP request failed with status: {}", final_response.status))
    }
}

#[update]
async fn add_credit_score(farm_id: u64, credit_score: u64, max_loan_amount: u64) -> Result<entitymanagement::Success, entitymanagement::Error> {
    let farmers = entitymanagement::return_farmers();
    let caller = ic_cdk::caller();

    // Check if the caller is a registered farmer
    let is_registered_farmer = farmers.iter().any(|farmer| farmer.principal_id == caller);

    if !is_registered_farmer {
        return Err(entitymanagement::Error::Error { 
            msg: String::from("Unauthorized: Caller is not a registered farmer") 
        });
    }

    let mut farmers = entitymanagement::return_farmers(); 

    // Transfer ifarm token to the caller
    // let amount = Nat::from(max_loan_amount); 
    // let _ = ifarm_tokens::ifarm_transfer(caller, amount).await; 

    if let Some(farm) = farmers.iter_mut().find(|f| f.id == farm_id) {
        // Additional check: Ensure the caller owns this farm
        if farm.principal_id != caller {
            return Err(entitymanagement::Error::Error { 
                msg: String::from("Unauthorized: Caller does not own this farm") 
            });
        }

        farm.credit_score = Some(credit_score); 
        farm.ifarm_tokens = Some(max_loan_amount);
        farm.max_loan_amount = Some(max_loan_amount);

        let farm_clone = farm.clone(); 
        entitymanagement::FARMER_STORAGE.with(|service| service.borrow_mut().insert(farm_id, farm_clone)); 

        Ok(entitymanagement::Success::CreditScoreAdded { 
            msg: format!("Credit score updated for farm_id: {}", farm_id)
        })
    } else {
        Err(entitymanagement::Error::Error { msg: String::from("Farm not found") })
    }
}