use std::collections::{HashMap, HashSet};

// Define a struct to represent a pending order
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct PendingOrder {
    farmer_id: u64,
    items: HashMap<String, u64>, // Map item name to quantity
}

// Define a struct to represent a completed order
#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct CompletedOrder {
    farmer_id: u64,
    items: HashMap<String, u64>, // Map item name to quantity
}
