use ic_cdk::caller;
use ic_cdk::{query, update, init};
use std::collections::HashMap;
use std::ptr::{addr_of, addr_of_mut};
use crate::entitymanagement::{self, get_registered_farmers, NewOrder, Order, OrderStatus, Success, SupplyAgriBusiness, Error};

// Global collection to store SupplyAgriBusiness instances
type SupplyAgriBusinesses = HashMap<u64, SupplyAgriBusiness>;
static mut SUPPLY_AGRIBUSINESSES: Option<SupplyAgriBusinesses> = None;

#[init]
fn init() {
    unsafe {
        SUPPLY_AGRIBUSINESSES = Some(HashMap::new());
    }
}

/**
* add_supply_items
* Adds supply items to a supply agribusiness if empty.
* @param supply_agribusiness_id: u64, items: Vec<SupplyItem>
* @return type: Result<Success, Error>
*/
#[update]
pub fn add_supply_items(
    supply_agribusiness_id: u64,
    items: Vec<(String, (u64, u64))>,
) -> Result<Success, Error> {
    entitymanagement::SUPPLY_AGRIBUSINESS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();

        if let Some(supply_agribusiness) = storage.get(&supply_agribusiness_id) {
            let mut supply_agribusiness = supply_agribusiness.clone();


            if supply_agribusiness.id != supply_agribusiness_id {
                return Err(Error::MismatchId { msg: "Mismatch in supply agribusiness ID.".to_string() });
            }
            
            if supply_agribusiness.items_to_be_supplied.is_none() {
                supply_agribusiness.items_to_be_supplied = Some(items);
                storage.insert(supply_agribusiness_id, supply_agribusiness);
                return Ok(Success::ItemsAdded {
                    msg: "Supply items added successfully.".to_string(),
                });
            } else {
                return Err(Error::ItemsNotEmpty {
                    msg: "Supply items already exist.".to_string(),
                });
            }
        } else {
            return Err(Error::AgribusinessNotFound {
                msg: "Supply agribusiness not found.".to_string(),
            });
        }
    })
}

/**
 * Function to create a new order
 * Creates a new order for a supply agribusiness based on the provided details.
 * @param new_order: NewOrder - The details of the new order.
 * @param supply_agribusiness_id: u64 - The ID of the supply agribusiness to create the order for.
 * @return bool - True if the order was successfully created, false otherwise.
 */
#[update]
pub fn create_order(new_order: NewOrder, supply_agribusiness_id: u64) -> bool {
    unsafe {
        if let Some(supply_agribusinesses) = addr_of_mut!(SUPPLY_AGRIBUSINESSES).as_mut().unwrap() {
            if let Some(supply_agribusiness) = supply_agribusinesses.get_mut(&supply_agribusiness_id) {
                if let Some(items_to_be_supplied) = supply_agribusiness.items_to_be_supplied.as_mut() {
                    for (item_name, (order_quantity, _)) in new_order.items.iter() {
                        if let Some((available_amount_str, item_price)) = items_to_be_supplied.iter_mut().find(|(name, _)| name == item_name) {
                            let available_amount = available_amount_str.parse::<u64>().unwrap_or(0);

                            if available_amount >= *order_quantity && supply_agribusiness.principal_id == caller() {
                                // Subtract the order quantity from the available amount
                                *available_amount_str = (available_amount - *order_quantity).to_string();

                                let order = Order {
                                    principal_id: caller(),
                                    order_id: 0,
                                    farmer_id: new_order.farmer_id,
                                    supply_agribusiness_id: new_order.supply_agribusiness_id,
                                    items: new_order.items.clone(),
                                    total_price: new_order.total_price,
                                    status: OrderStatus::Pending,
                                };

                                supply_agribusiness.orders.push(order);

                                let mut farmers = get_registered_farmers();
                                if let Some(farmer) = farmers.get_mut(&new_order.farmer_id.to_string()) {
                                    if let Some(farm_assets) = &mut farmer.farm_assets {
                                        let mut item_found = false;
                                        for (asset_name, (amount, value)) in farm_assets.iter_mut() {
                                            if asset_name == item_name {
                                                *amount += *order_quantity;
                                                *value += *order_quantity * item_price.1;
                                                item_found = true;
                                                break;
                                            }
                                        }
                                        if !item_found {
                                            farm_assets.push((item_name.clone(), (*order_quantity, *order_quantity * item_price.1)));
                                        }
                                    } else {
                                        farmer.farm_assets = Some(vec![(item_name.clone(), (*order_quantity, *order_quantity * item_price.1))]);
                                    }
                                }

                                let order_id = supply_agribusiness.orders.len() as u64 - 1;
                                update_order_status(order_id, OrderStatus::Complete, supply_agribusiness_id);

                                return true;
                            }
                        }
                    }
                }
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
 * @param supply_agribusiness_id: u64 - The ID of the supply agribusiness containing the order.
 * @return bool - True if the status was successfully updated, false otherwise.
*/
#[update]
pub fn update_order_status(order_id: u64, new_status: OrderStatus, supply_agribusiness_id: u64) -> bool {
    unsafe {
        if let Some(supply_agribusinesses) = addr_of_mut!(SUPPLY_AGRIBUSINESSES).as_mut().unwrap() {
            if let Some(supply_agribusiness) = supply_agribusinesses.get_mut(&supply_agribusiness_id) {
                if let Some(order) = supply_agribusiness.orders.iter_mut().find(|order| order.order_id == order_id) {
                    order.status = new_status;
                    return true;
                }
            }
        }
    }
    false
}

/**
 * Function to list orders by a supply agribusiness
 * Retrieves a list of orders associated with a supply agribusiness.
 * @param supply_agribusiness_id: u64 - The ID of the supply agribusiness to list orders for.
 * @return Vec<Order> - A vector containing orders associated with the supply agribusiness.
 */
#[query]
pub fn list_orders_by_agribusiness(supply_agribusiness_id: u64) -> Vec<Order> {
    unsafe {
        if let Some(supply_agribusinesses) = addr_of!(SUPPLY_AGRIBUSINESSES).as_ref().unwrap() {
            if let Some(supply_agribusiness) = supply_agribusinesses.get(&supply_agribusiness_id) {
                return supply_agribusiness.orders.clone();
            }
        }
    }
    Vec::new()
}

/**
 * Function to list orders sent by a farmer to supply agribusinesses
 * Retrieves a list of orders sent by a specific farmer to multiple supply agribusinesses.
 * @param farmer_id: u64 - The ID of the farmer to list orders for.
 * @param supply_agribusiness_ids: Vec<u64> - A vector of supply agribusiness IDs to search for orders.
 * @return Vec<Order> - A vector containing orders sent by the farmer to the supply agribusinesses.
*/
#[query]
pub fn list_farmer_sent_orders(farmer_id: u64, supply_agribusiness_ids: Vec<u64>) -> Vec<Order> {
    let mut farmer_orders: Vec<Order> = Vec::new();
    unsafe {
        if let Some(supply_agribusinesses) = addr_of!(SUPPLY_AGRIBUSINESSES).as_ref().unwrap() {
            for &id in &supply_agribusiness_ids {
                if let Some(supply_agribusiness) = supply_agribusinesses.get(&id) {
                    for order in &supply_agribusiness.orders {
                        if order.farmer_id == farmer_id {
                            farmer_orders.push(order.clone());
                        }
                    }
                }
            }
        }
    }
    farmer_orders
}
