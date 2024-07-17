use ic_cdk::caller;
use ic_cdk::{query, update};
use crate::entitymanagement::{Order, NewOrder, OrderStatus, SupplyAgriBusiness};


/**
* Function to create a new order
* Creates a new order for a supply agribusiness based on the provided details.
* @param new_order: NewOrder - The details of the new order.
* @param supply_agribusiness: &mut SupplyAgriBusiness - The supply agribusiness to create the order for.
* @return bool - True if the order was successfully created, false otherwise.
*/
#[update]
pub fn create_order(new_order: NewOrder, mut supply_agribusiness: SupplyAgriBusiness) -> bool {
    // Unwrap the Option to get access to the Vec inside items_to_be_supplied
    if let Some(items_to_be_supplied) = supply_agribusiness.items_to_be_supplied.as_mut() {
        // Attempt to find the item in items_to_be_supplied
        if let Some((available_amount_str, item_price)) = items_to_be_supplied.iter_mut().find(|(item_name, _)| new_order.items.contains_key(item_name)) {
            // Convert available_amount from String to u64
            let available_amount = available_amount_str.parse::<u64>().unwrap_or(0);

            // Calculate total price
            let total_price = item_price.1 * new_order.total_price;

            // Ensure enough amount available and caller is authorized
            if available_amount >= new_order.total_price && supply_agribusiness.principal_id == caller() {
                // Subtract new_order.total_price from available_amount and convert back to String
                *available_amount_str = (available_amount - new_order.total_price).to_string();

                // Create the order
                let order = Order {
                    principal_id: caller(),
                    order_id: 0, // Assign a unique order ID here
                    farmer_id: new_order.farmer_id,
                    supply_agribusiness_id: new_order.supply_agribusiness_id,
                    items: new_order.items.clone(), // Clone the items HashMap
                    total_price: new_order.total_price,
                    status: OrderStatus::Pending, // Default status is Pending
                };

                supply_agribusiness.orders.push(order);

                return true;
            }
        }
    }
    false
}

/**
* Function to update the status of an order
* Updates the status of an order for a supply agribusiness.
* @param order_id: u64 - The ID of the order to update.
* @param new_status: OrderStatus - The new status to set for the order.
* @param supply_agribusiness: &mut SupplyAgriBusiness - The supply agribusiness containing the order.
* @return bool - True if the status was successfully updated, false otherwise.
*/
#[update]
pub fn update_order_status(order_id: u64, new_status: OrderStatus, mut supply_agribusiness: SupplyAgriBusiness) -> bool {
    // Find the order and update its status
    if let Some(order) = supply_agribusiness.orders.iter_mut().find(|order| order.order_id == order_id) {
        order.status = new_status;
        return true;
    }
    false
}

/**
* Function to list orders by a supply agribusiness
* Retrieves a list of orders associated with a supply agribusiness.
* @param supply_agribusiness: &SupplyAgriBusiness - The supply agribusiness to list orders for.
* @return Vec<Order> - A vector containing orders associated with the supply agribusiness.
*/
#[query]
pub fn list_orders_by_agribusiness(supply_agribusiness: SupplyAgriBusiness) -> Vec<Order> {
    // Return orders associated with a supply agribusiness
    supply_agribusiness.orders
}

/**
* Function to list orders sent by a farmer to supply agribusinesses
* Retrieves a list of orders sent by a specific farmer to multiple supply agribusinesses.
* @param farmer_id: u64 - The ID of the farmer to list orders for.
* @param supply_agribusinesses: Vec<SupplyAgriBusiness> - A vector of supply agribusinesses to search for orders.
* @return Vec<Order> - A vector containing orders sent by the farmer to the supply agribusinesses.
*/
#[query]
pub fn list_farmer_sent_orders(farmer_id: u64, supply_agribusinesses: Vec<SupplyAgriBusiness>) -> Vec<Order> {
    let mut farmer_orders: Vec<Order> = Vec::new();

    // Iterate through supply agribusinesses and their orders
    for supply_agribusiness in supply_agribusinesses {
        for order in supply_agribusiness.orders {
            if order.farmer_id == farmer_id {
                farmer_orders.push(order);
            }
        }
    }

    farmer_orders
}
