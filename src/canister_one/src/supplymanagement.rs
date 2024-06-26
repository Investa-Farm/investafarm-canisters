use ic_cdk::update;
use crate::entitymanagement::{self, Order, NewOrder, OrderStatus, Error, Success};
use std::collections::HashMap;

/**
 * CreateOrder
 * Creates an order for a farmer from a supply agribusiness
 * @params farmer: &mut Farmer, supply_agribusiness: &mut SupplyAgriBusiness, new_order: NewOrder
 * @return Result<(), String>
 */
 pub fn create_order(
    farmer: &mut Farmer,
    supply_agribusiness: &mut SupplyAgriBusiness,
    new_order: NewOrder,
) -> Result<(), String> {
    // Check if the supply agribusiness has the item in the required amount
    let item_price = match supply_agribusiness.items.get(&new_order.item_name) {
        Some(&price) => price,
        None => return Err(format!("Item '{}' not found in supply agribusiness.", &new_order.item_name)),
    };

    if new_order.amount > supply_agribusiness.stock.get(&new_order.item_name).copied().unwrap_or(0) {
        return Err(format!(
            "Not enough stock for '{}'. Requested: {}, Available: {}",
            &new_order.item_name, new_order.amount, supply_agribusiness.stock[&new_order.item_name]
        ));
    }

    let total_price = item_price * new_order.amount;
    if total_price > farmer.loan {
        return Err("Insufficient loan amount to place the order.".to_string());
    }

    // Deduct the items from the stock
    *supply_agribusiness.stock.get_mut(&new_order.item_name).unwrap() -= new_order.amount;

    // Update farmer loan and assets
    farmer.loan -= total_price;
    farmer.assets.entry(new_order.item_name.clone()).and_modify(|e| *e += new_order.amount).or_insert(new_order.amount);

    // Create and add the order
    let order = Order {
        item_name: new_order.item_name,
        amount: new_order.amount,
        total_price,
        status: OrderStatus::Pending,
    };
    farmer.orders.push(order.clone());
    supply_agribusiness.orders.push(order);

    Ok(())
}

/**
 * UpdateOrderStatus
 * Updates the status of an existing order
 * @params order: &mut Order, new_status: OrderStatus
 * @return ()
 */
pub fn update_order_status(order: &mut Order, new_status: OrderStatus) {
    order.status = new_status;
}

/**
 * ListSupplyAgribusinessOrders
 * Lists all orders associated with a specific supply agribusiness
 * @params supply_agribusiness: &SupplyAgriBusiness
 * @return Vec<Order>
 */
pub fn list_supply_agribusiness_orders(supply_agribusiness: &SupplyAgriBusiness) -> Vec<Order> {
    supply_agribusiness.orders.clone()
}

/**
 * ListFarmerOrders
 * Lists all orders sent by a specific farmer
 * @params farmer: &Farmer
 * @return Vec<Order>
 */
pub fn list_farmer_orders(farmer: &Farmer) -> Vec<Order> {
    farmer.orders.clone()
}