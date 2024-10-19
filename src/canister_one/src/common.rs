use evm_rpc_canister_types::{
    EthSepoliaService, GetTransactionReceiptResult, MultiGetTransactionReceiptResult, RpcServices,
    EvmRpcCanister,
};
use candid::Nat;
use hex;
use num_traits::cast::ToPrimitive;
use num_bigint::BigUint;      

pub async fn eth_get_transaction_receipt(
    evm_rpc: &EvmRpcCanister,
    hash: String,
) -> Result<GetTransactionReceiptResult, String> {
    let result: Result<(MultiGetTransactionReceiptResult,), String> = evm_rpc
        .eth_get_transaction_receipt(
            RpcServices::EthSepolia(Some(vec![
                EthSepoliaService::PublicNode,
                EthSepoliaService::BlockPi,
                EthSepoliaService::Ankr,
            ])),
            None,
            hash,
            10_000_000_000,
        )
        .await
        .map_err(|e| format!("Failed to call eth_getTransactionReceipt: {:?}", e));

    match result {
        Ok((MultiGetTransactionReceiptResult::Consistent(receipt),)) => Ok(receipt),
        Ok((MultiGetTransactionReceiptResult::Inconsistent(error),)) => {
            Err(format!("EVM_RPC returned inconsistent results: {:?}", error))
        }
        Err(e) => Err(format!("Error calling EVM_RPC: {}", e)),
    }
}

pub fn hex_string_with_0x_to_f64(hex_string: String) -> f64 {
    let hex_string = hex_string.trim_start_matches("0x");
    let bytes = hex::decode(hex_string).expect("Failed to decode hex string");
    let big_uint = BigUint::from_bytes_be(&bytes);
    let nat = Nat::from(big_uint);
    nat.0.to_f64().unwrap_or(f64::MAX)
}