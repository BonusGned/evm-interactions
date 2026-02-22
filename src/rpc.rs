use crate::model::{Block, JsonRpcResponse};
use reqwest::Client;
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

    pub async fn get_latest_block(&self, rpc_url: &str) -> Result<Block, String> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": ["latest", false],
            "id": 1
        });

        let response = self
            .client
            .post(rpc_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("request failed: {e}"))?;

        let rpc_response: JsonRpcResponse = response
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
}
