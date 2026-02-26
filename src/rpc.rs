use crate::model::{Block, JsonRpcResponse, Transaction, TransactionReceipt};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::json;
use std::time::Duration;

#[derive(Clone)]
pub struct RpcClient {
    client: Client,
}

impl RpcClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("failed to build HTTP client"),
        }
    }

    async fn call<T: DeserializeOwned>(
        &self,
        rpc_url: &str,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, String> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let response = self
            .client
            .post(rpc_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("request failed: {e}"))?;

        let rpc_response: JsonRpcResponse<T> = response
            .json()
            .await
            .map_err(|e| format!("failed to parse response: {e}"))?;

        if let Some(err) = rpc_response.error {
            return Err(format!("RPC error {}: {}", err.code, err.message));
        }

        rpc_response
            .result
            .ok_or_else(|| "empty result".to_string())
    }

    pub async fn get_block(&self, rpc_url: &str, block_id: &str) -> Result<Block, String> {
        self.call(rpc_url, "eth_getBlockByNumber", json!([block_id, false]))
            .await
    }

    pub async fn get_transaction(&self, rpc_url: &str, hash: &str) -> Result<Transaction, String> {
        self.call(rpc_url, "eth_getTransactionByHash", json!([hash]))
            .await
    }

    pub async fn get_transaction_receipt(
        &self,
        rpc_url: &str,
        hash: &str,
    ) -> Result<TransactionReceipt, String> {
        self.call(rpc_url, "eth_getTransactionReceipt", json!([hash]))
            .await
    }

    pub async fn get_balance(&self, rpc_url: &str, address: &str) -> Result<String, String> {
        self.call(rpc_url, "eth_getBalance", json!([address, "latest"]))
            .await
    }

    pub async fn get_gas_price(&self, rpc_url: &str) -> Result<String, String> {
        self.call(rpc_url, "eth_gasPrice", json!([])).await
    }

    pub async fn get_max_priority_fee(&self, rpc_url: &str) -> Result<String, String> {
        self.call(rpc_url, "eth_maxPriorityFeePerGas", json!([]))
            .await
    }
}
