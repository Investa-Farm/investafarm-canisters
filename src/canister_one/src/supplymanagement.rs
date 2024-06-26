use ic_cdk::{query, update};
use std::collections::HashMap;
use crate::entitymanagement::{self, NewOrder, OrderStatus, SupplyAgriBusiness};

/**
* create_order
* Creates a new order for supply items from a farmer to a supply agribusiness.
* @params
*   - new_order: NewOrder - Struct containing details of the order to be created.
*   - supply_agribusiness: &mut SupplyAgriBusiness - Mutable reference to the supply agribusiness.
* @return bool - Indicates if the order creation was successful.
*/
#[update]
pub fn create_order(new_order: NewOrder, supply_agribusiness: &mut SupplyAgriBusiness) -> bool {
    // Check if the item exists in supply_items and get its availability and price
    if let Some((available_amount, item_price)) = supply_agribusiness.supply_items.get_mut(&new_order.item_name) {
        // Calculate total price of the order
        let total_price = item_price * new_order.amount;

        // Check if there's enough amount available and if loan ask is sufficient
        if *available_amount >= new_order.amount && supply_agribusiness.principal_id > total_price {
            // Update available amount
            *available_amount -= new_order.amount;

            // Update farmer's loan ask
            supply_agribusiness.principal_id -= total_price;

            // Add the order to the supply agribusiness's orders list
            supply_agribusiness.orders.push(new_order);

            return true;
        }
    }
    false
}

/**
* update_order_status
* Updates the status of an existing order.
* @params
*   - order_id: u64 - Identifier of the order to update.
*   - new_status: OrderStatus - New status to assign to the order.
*   - supply_agribusiness: &mut SupplyAgriBusiness - Mutable reference to the supply agribusiness.
* @return bool - Indicates if the status update was successful.
*/
#[update] 
pub fn update_order_status(order_id: u64, new_status: bool, supply_agribusiness: &mut SupplyAgriBusiness) -> bool {
    for order in &mut supply_agribusiness.orders {
        if order.order_id == order_id {
            order.status = new_status;
            return true;
        }
    }
    false
}

/**
* list_orders_by_agribusiness
* Lists all orders associated with a specific supply agribusiness.
* @params
*   - supply_agribusiness: &SupplyAgriBusiness - Reference to the supply agribusiness.
* @return Vec<&NewOrder> - Vector of references to orders associated with the supply agribusiness.
*/
#[query] 
pub fn list_orders_by_agribusiness(supply_agribusiness: &SupplyAgriBusiness) -> Vec<&NewOrder> {
    supply_agribusiness.orders.iter().collect()
}

/**
* list_farmer_sent_orders
* Lists all orders sent by a specific farmer.
* @params
*   - farmer_id: u64 - Identifier of the farmer.
*   - supply_agribusinesses: &[SupplyAgriBusiness] - Slice of supply agribusinesses to search for orders.
* @return Vec<&NewOrder> - Vector of references to orders sent by the farmer across all agribusinesses.
*/
#[query] 
pub fn list_farmer_sent_orders(farmer_id: u64, supply_agribusinesses: &[SupplyAgriBusiness]) -> Vec<&NewOrder> {
    let mut farmer_orders: Vec<&NewOrder> = Vec::new();

    // Iterate through each supply agribusiness
    for supply_agribusiness in supply_agribusinesses {
        // Iterate through orders of each supply agribusiness
        for order in &supply_agribusiness.orders {
            if order.farmer_id == farmer_id {
                farmer_orders.push(order);
            }
        }
    }
}