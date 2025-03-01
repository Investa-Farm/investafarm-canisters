use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext, TransformFunc
};
use crate::entitymanagement::{ 
    return_farmers, 
    return_investors, 
    return_supply_agribusiness, 
    return_farms_agribusiness
}; 
use ic_cdk::{ update, heartbeat}; 

// Helper function to backup data to Supabase
async fn backup_to_supabase(data: Vec<u8>, table: &str) -> Result<(), String> {
    let supabase_url = "https://jrlqttqoaaiuaimuuxfw.supabase.co/functions/v1/backup-data";

    let request_headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "Idempotency-Key".to_string(),
            value: format!("backup-{}-{}", table, ic_cdk::api::time()),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: supabase_url.to_string(),
        method: HttpMethod::POST,
        body: Some(data),
        max_response_bytes: None,
        transform: Some(TransformContext {
            function: TransformFunc(candid::Func {
                principal: ic_cdk::api::id(),
                method: "transform".to_string(),
            }),
            context: vec![],
        }),
        headers: request_headers,
    };

    let cycles: u128 = 2_000_000_000_000;

    match http_request(request, cycles).await {
        Ok((response,)) => {
            if response.status == 200u16 || response.status == 201u16 {
                Ok(())
            } else {
                Err(format!("Backup failed with status: {}", response.status))
            }
        }
        Err((r, m)) => {
            Err(format!("Failed to backup data. Error: {:?}, Message: {}", r, m))
        }
    }
}

// Function to backup farmers to Supabase
#[update]
pub async fn backup_farmers() -> Result<(), String> {
    let farmers = return_farmers();
    let json = serde_json::to_vec(&farmers)
        .map_err(|e| format!("Failed to serialize farmers: {}", e))?;
    
    backup_to_supabase(json, "farmers").await
}

// Function to backup investors to Supabase
#[update] 
pub async fn backup_investors() -> Result<(), String> {
    let investors = return_investors();
    let json = serde_json::to_vec(&investors)
        .map_err(|e| format!("Failed to serialize investors: {}", e))?;

    backup_to_supabase(json, "investors").await
}

// Function to backup supply agribusinesses to Supabase
#[update]
pub async fn backup_supply_agribusinesses() -> Result<(), String> {
    let supply_agribusinesses = return_supply_agribusiness();
    let json = serde_json::to_vec(&supply_agribusinesses)
        .map_err(|e| format!("Failed to serialize supply agribusinesses: {}", e))?;
    
    backup_to_supabase(json, "supply_agribusinesses").await
}

// Function to backup farms agribusinesses to Supabase
#[update]
pub async fn backup_farms_agribusinesses() -> Result<(), String> {
    let farms_agribusinesses = return_farms_agribusiness();
    let json = serde_json::to_vec(&farms_agribusinesses)
        .map_err(|e| format!("Failed to serialize farms agribusinesses: {}", e))?;

    backup_to_supabase(json, "farms_agribusinesses").await
}

// Function to backup all data to Supabase
#[update]
pub async fn backup_all_data() -> Result<(), String> {
    backup_farmers().await?;
    backup_investors().await?;
    backup_supply_agribusinesses().await?;
    backup_farms_agribusinesses().await?;
    Ok(())
}

// Periodic backup function
#[heartbeat]
fn periodic_backup() {
    ic_cdk::spawn(async {
        match backup_all_data().await {
            Ok(_) => ic_cdk::println!("Backup completed successfully"),
            Err(e) => ic_cdk::println!("Backup failed: {}", e)
        }
    });
}