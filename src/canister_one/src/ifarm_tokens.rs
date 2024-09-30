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
// use crate::LEDGER;

const IFARM_TOKEN: &str = "lradw-laaaa-aaaam-acrda-cai";

// Check ifarm token balance
#[ic_cdk::update]
async fn ifarm_balance(principal_id: Principal) -> Nat {
    let account = ICRCAccount::new(principal_id, None);
    ICRC1::from(IFARM_TOKEN).balance_of(account).await.unwrap()
}   

// Transfer ifarm token
#[ic_cdk::update]
pub async fn ifarm_transfer(to: Principal, amount: Nat) -> ICRC1TransferResult {
    let to = ICRCAccount::new(to, None);
    let transfer_args = ICRC1TransferArgs {
        to,
        amount,
        from_subaccount: None,
        fee: None,
        memo: None,
        created_at_time: None,
    };
    ICRC1::from(IFARM_TOKEN).transfer(transfer_args).await.unwrap()
}

// Approve ifarm token
#[ic_cdk::update]
async fn ifarm_approve(spender: Principal, amount: Nat) -> ICRC2ApproveResult {
    let spender = ICRCAccount::new(spender, None);
    let approve_args = ICRC2ApproveArgs {
        spender,
        amount,
        from_subaccount: None,
        expected_allowance: None,
        expires_at: None,
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