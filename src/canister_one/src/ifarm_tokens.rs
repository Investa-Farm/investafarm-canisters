use b3_utils::ledger::{
    ICRCAccount, 
    ICRC1, 
    ICRC1TransferArgs, 
    ICRC1TransferResult, 
    ICRC2ApproveArgs, 
    ICRC2ApproveResult, 
    ICRC2, 
    ICRC2TransferFromArgs, 
    ICRC2TransferFromResult, 
    // ICRC2AllowanceArgs, 
    // ICRC2Allowance
};
use candid::{Principal, Nat};
use num_traits::ToPrimitive;
// use crate::LEDGER;

const IFARM_TOKEN: &str = "lradw-laaaa-aaaam-acrda-cai";
const FEE_COLLECTOR_PRINCIPAL: &str = "3r4ur-bi57q-dnrjp-fdl3f-pd5ud-gux43-l6bk6-ff7p3-33zk4-nx7ym-mqe";
const FEE_PERCENTAGE: f64 = 0.01; 

async fn collect_fee(amount: &Nat) -> ICRC1TransferResult {
    let fee_amount = (amount.0.to_f64().unwrap() * FEE_PERCENTAGE) as u64;
    let fee_collector = Principal::from_text(FEE_COLLECTOR_PRINCIPAL).unwrap();
    
    Box::pin(ifarm_transfer(fee_collector, Nat::from(fee_amount))).await
}

// Check ifarm token balance
#[ic_cdk::update]
async fn ifarm_balance(principal_id: Principal) -> Nat {
    let account = ICRCAccount::new(principal_id, None);
    ICRC1::from(IFARM_TOKEN).balance_of(account).await.unwrap()
}   

// Transfer ifarm token
#[ic_cdk::update]
pub async fn ifarm_transfer(to: Principal, amount: Nat) -> ICRC1TransferResult {
    // First collect the fee
    let fee_amount = (amount.0.to_f64().unwrap() * FEE_PERCENTAGE) as u64;
    let _ = collect_fee(&amount).await;

    // Calculate remaining amount after fee deduction
    let transfer_amount = amount - Nat::from(fee_amount);

    let to = ICRCAccount::new(to, None);
    let transfer_args = ICRC1TransferArgs {
        to,
        amount: transfer_amount,
        from_subaccount: None,
        // fee: Some(Nat::from(10000u64)),
        fee: None,
        memo: None,
        created_at_time: None,
    };
    ICRC1::from(IFARM_TOKEN).transfer(transfer_args).await.unwrap()
}

// Approve ifarm token
#[ic_cdk::update]
async fn ifarm_approve(spender: Principal, amount: Nat) -> ICRC2ApproveResult {
    // let subaccount = ICRCAccount::new(subaccount, None);
    let spender = ICRCAccount::new(spender, None);
    let approve_args = ICRC2ApproveArgs {
        spender,
        amount,
        from_subaccount: None,
        expected_allowance: None,
        expires_at: None,
        // fee: Some(Nat::from(10000u64)),
        fee: None,
        memo: None,
        created_at_time: None,
    };
    ICRC2::from(IFARM_TOKEN).approve(approve_args).await.unwrap()
}

// Transfer ifarm token
#[ic_cdk::update]
async fn ifarm_transfer_from(from: Principal, to: Principal, amount: Nat) -> ICRC2TransferFromResult {
    let from = ICRCAccount::new(from, None);
    let to = ICRCAccount::new(to, None);
    let transfer_from_args = ICRC2TransferFromArgs {
        from, 
        to,
        amount,
        spender_subaccount: None,
        // fee: Some(Nat::from(10000u64)),
        fee: None,
        memo: None,
        created_at_time: None,
    };
    // ICRC2::from(IFARM_TOKEN).transfer_from(transfer_from_args).await.map_err(|e| e.into())
    ICRC2::from(IFARM_TOKEN).transfer_from(transfer_from_args).await.unwrap()
}

// Check token allowance
// #[ic_cdk::update]
// async fn ifarm_allowance(account: Principal, spender: Principal) -> ICRC2Allowance {
//     let account = ICRCAccount::new(account, None);
//     let spender = ICRCAccount::new(spender, None);
//     let allowance_args = ICRC2AllowanceArgs {
//         account,
//         spender,
//     };
//     ICRC2::from(IFARM_TOKEN).allowance(allowance_args).await.unwrap()
// }