use ic_cdk::api::management_canister::http_request::{HttpResponse, HttpMethod, CanisterHttpRequestArgument, http_request};
// use ic_cdk::export::Principal;
use ic_cdk_macros::update;
use serde_json::Value;
use num_bigint::BigUint;

const REQUIRED_CYCLES: u64 = 1_603_078_400; // Adjust this value as needed

#[update]
async fn fetch_usd_to_kes() -> Result<f64, String> {
    let url = "https://api.exchangerate-api.com/v4/latest/USD";
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        headers: vec![],
        body: None,
        max_response_bytes: Some(2_000_000),
        transform: None,
    };

    let (response,): (HttpResponse,) = http_request(request, REQUIRED_CYCLES.into()).await.map_err(|e| format!("HTTP request failed: {:?}", e))?;

    if response.status.0 == BigUint::from(200u64) {
        let response_body: Value = serde_json::from_slice(&response.body)
            .map_err(|e| format!("Failed to parse response: {:?}", e))?;
        
        if let Some(rate) = response_body.get("rates").and_then(|rates| rates.get("KES")).and_then(|v| v.as_f64()) {
            Ok(rate)
        } else {
            Err("KES rate not found in response".to_string())
        }
    } else {
        Err(format!("HTTP request failed with status: {}", response.status))
    }
}

#[update]
async fn fetch_kes_to_usd() -> Result<f64, String> {
    let url = "https://api.exchangerate-api.com/v4/latest/KES";
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        headers: vec![],
        body: None,
        max_response_bytes: Some(2_000_000),
        transform: None,
    };

    let (response,): (HttpResponse,) = http_request(request, REQUIRED_CYCLES.into()).await.map_err(|e| format!("HTTP request failed: {:?}", e))?;

    if response.status.0 == BigUint::from(200u64) {
        let response_body: Value = serde_json::from_slice(&response.body)
            .map_err(|e| format!("Failed to parse response: {:?}", e))?;
        
        if let Some(rate) = response_body.get("rates").and_then(|rates| rates.get("USD")).and_then(|v| v.as_f64()) {
            Ok(rate)
        } else {
            Err("USD rate not found in response".to_string())
        }
    } else {
        Err(format!("HTTP request failed with status: {}", response.status))
    }
}