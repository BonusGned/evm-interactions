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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn block_response() -> serde_json::Value {
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "number": "0x1234",
                "hash": "0xabcdef",
                "timestamp": "0x60000000",
                "gasUsed": "0x5208",
                "gasLimit": "0x1c9c380",
                "baseFeePerGas": "0x3b9aca00",
                "transactions": [],
                "miner": "0x0000000000000000000000000000000000000000"
            }
        })
    }

    fn tx_response() -> serde_json::Value {
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "hash": "0xtxhash",
                "blockNumber": "0x100",
                "from": "0xsender",
                "to": "0xreceiver",
                "value": "0xde0b6b3a7640000",
                "gas": "0x5208",
                "gasPrice": "0x3b9aca00",
                "input": "0x",
                "nonce": "0x1",
                "type": "0x2"
            }
        })
    }

    fn receipt_response() -> serde_json::Value {
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "status": "0x1",
                "gasUsed": "0x5208",
                "effectiveGasPrice": "0x3b9aca00",
                "contractAddress": null
            }
        })
    }

    #[tokio::test]
    async fn test_rpc_get_block_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(block_response()))
            .mount(&server)
            .await;

        let client = RpcClient::new();
        let block = client.get_block(&server.uri(), "latest").await.unwrap();
        assert_eq!(block.number, "0x1234");
        assert_eq!(block.gas_used, "0x5208");
    }

    #[tokio::test]
    async fn test_rpc_get_block_rpc_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": { "code": -32602, "message": "invalid params" }
            })))
            .mount(&server)
            .await;

        let client = RpcClient::new();
        let result = client.get_block(&server.uri(), "0xINVALID").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("RPC error"));
        assert!(err.contains("invalid params"));
    }

    #[tokio::test]
    async fn test_rpc_get_block_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": null
            })))
            .mount(&server)
            .await;

        let client = RpcClient::new();
        let result = client.get_block(&server.uri(), "0xffffff").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty result"));
    }

    #[tokio::test]
    async fn test_rpc_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&server)
            .await;

        let client = RpcClient::new();
        let result = client.get_block(&server.uri(), "latest").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("failed to parse response"));
    }

    #[tokio::test]
    async fn test_rpc_connection_refused() {
        let client = RpcClient::new();
        let result = client.get_block("http://127.0.0.1:1", "latest").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("request failed"));
    }

    #[tokio::test]
    async fn test_rpc_get_transaction_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(tx_response()))
            .mount(&server)
            .await;

        let client = RpcClient::new();
        let tx = client
            .get_transaction(&server.uri(), "0xtxhash")
            .await
            .unwrap();
        assert_eq!(tx.hash, "0xtxhash");
        assert_eq!(tx.from, "0xsender");
    }

    #[tokio::test]
    async fn test_rpc_get_transaction_receipt_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(receipt_response()))
            .mount(&server)
            .await;

        let client = RpcClient::new();
        let receipt = client
            .get_transaction_receipt(&server.uri(), "0xtxhash")
            .await
            .unwrap();
        assert_eq!(receipt.status.as_deref(), Some("0x1"));
        assert_eq!(receipt.gas_used, "0x5208");
    }

    #[tokio::test]
    async fn test_rpc_get_balance_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0xde0b6b3a7640000"
            })))
            .mount(&server)
            .await;

        let client = RpcClient::new();
        let balance = client
            .get_balance(&server.uri(), "0xaddress")
            .await
            .unwrap();
        assert_eq!(balance, "0xde0b6b3a7640000");
    }

    #[tokio::test]
    async fn test_rpc_get_gas_price_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x3b9aca00"
            })))
            .mount(&server)
            .await;

        let client = RpcClient::new();
        let gas = client.get_gas_price(&server.uri()).await.unwrap();
        assert_eq!(gas, "0x3b9aca00");
    }

    #[tokio::test]
    async fn test_rpc_get_max_priority_fee_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x77359400"
            })))
            .mount(&server)
            .await;

        let client = RpcClient::new();
        let fee = client.get_max_priority_fee(&server.uri()).await.unwrap();
        assert_eq!(fee, "0x77359400");
    }
}
