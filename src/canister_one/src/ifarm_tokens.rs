use b3_utils::ledger::{ICRCAccount, ICRC1, ICRC1TransferArgs, ICRC1TransferResult};
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
async fn ifarm_transfer(to: Principal, amount: Nat) -> ICRC1TransferResult {
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

