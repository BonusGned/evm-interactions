use serde::Deserialize;

fn hex_to_u64(hex: &str) -> u64 {
    u64::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0)
}

fn hex_to_u128(hex: &str) -> u128 {
    u128::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0)
}

pub fn wei_hex_to_ether(hex: &str) -> f64 {
    hex_to_u128(hex) as f64 / 1e18
}

pub fn wei_hex_to_gwei(hex: &str) -> f64 {
    hex_to_u128(hex) as f64 / 1e9
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub number: String,
    pub hash: String,
    pub timestamp: String,
    pub gas_used: String,
    pub gas_limit: String,
    pub base_fee_per_gas: Option<String>,
    pub transactions: Vec<serde_json::Value>,
    pub miner: String,
}

impl Block {
    pub fn number_dec(&self) -> u64 {
        hex_to_u64(&self.number)
    }

    pub fn timestamp_dec(&self) -> i64 {
        i64::from_str_radix(self.timestamp.trim_start_matches("0x"), 16).unwrap_or(0)
    }

    pub fn gas_used_dec(&self) -> u64 {
        hex_to_u64(&self.gas_used)
    }

    pub fn gas_limit_dec(&self) -> u64 {
        hex_to_u64(&self.gas_limit)
    }

    pub fn base_fee_gwei(&self) -> Option<f64> {
        self.base_fee_per_gas
            .as_ref()
            .map(|hex| hex_to_u128(hex) as f64 / 1e9)
    }

    pub fn gas_usage_percent(&self) -> f64 {
        let limit = self.gas_limit_dec();
        if limit == 0 {
            return 0.0;
        }
        (self.gas_used_dec() as f64 / limit as f64) * 100.0
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub hash: String,
    pub block_number: Option<String>,
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub gas: String,
    pub gas_price: Option<String>,
    pub input: String,
    pub nonce: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    pub tx_type: Option<String>,
}

impl Transaction {
    pub fn value_ether(&self) -> f64 {
        hex_to_u128(&self.value) as f64 / 1e18
    }

    pub fn gas_limit_dec(&self) -> u64 {
        hex_to_u64(&self.gas)
    }

    pub fn gas_price_gwei(&self) -> Option<f64> {
        self.gas_price
            .as_ref()
            .map(|hex| hex_to_u128(hex) as f64 / 1e9)
    }

    pub fn nonce_dec(&self) -> u64 {
        hex_to_u64(&self.nonce)
    }

    pub fn block_number_dec(&self) -> Option<u64> {
        self.block_number.as_ref().map(|hex| hex_to_u64(hex))
    }

    pub fn input_preview(&self) -> &str {
        if self.input == "0x" || self.input.is_empty() {
            "0x (transfer)"
        } else if self.input.len() > 10 {
            &self.input[..10]
        } else {
            &self.input
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub status: Option<String>,
    pub gas_used: String,
    pub effective_gas_price: Option<String>,
    pub contract_address: Option<String>,
}

impl TransactionReceipt {
    pub fn succeeded(&self) -> Option<bool> {
        self.status.as_ref().map(|s| s == "0x1")
    }

    pub fn gas_used_dec(&self) -> u64 {
        hex_to_u64(&self.gas_used)
    }

    pub fn effective_gas_price_gwei(&self) -> Option<f64> {
        self.effective_gas_price
            .as_ref()
            .map(|hex| hex_to_u128(hex) as f64 / 1e9)
    }

    pub fn tx_cost_ether(&self) -> f64 {
        let gas_price = self
            .effective_gas_price
            .as_ref()
            .map(|hex| hex_to_u128(hex))
            .unwrap_or(0);
        (gas_price * self.gas_used_dec() as u128) as f64 / 1e18
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_u64() {
        assert_eq!(hex_to_u64("0xff"), 255);
        assert_eq!(hex_to_u64("0x0"), 0);
        assert_eq!(hex_to_u64("0x1234"), 0x1234);
        assert_eq!(hex_to_u64("invalid"), 0);
    }

    #[test]
    fn test_hex_to_u128() {
        assert_eq!(hex_to_u128("0xde0b6b3a7640000"), 1_000_000_000_000_000_000);
        assert_eq!(hex_to_u128("0x0"), 0);
        assert_eq!(hex_to_u128("invalid"), 0);
    }

    #[test]
    fn test_wei_hex_to_ether() {
        assert!((wei_hex_to_ether("0xde0b6b3a7640000") - 1.0).abs() < 1e-10);
        assert_eq!(wei_hex_to_ether("0x0"), 0.0);
    }

    #[test]
    fn test_wei_hex_to_gwei() {
        assert!((wei_hex_to_gwei("0x3b9aca00") - 1.0).abs() < 1e-10);
        assert_eq!(wei_hex_to_gwei("0x0"), 0.0);
    }

    fn make_block() -> Block {
        Block {
            number: "0x100".to_string(),
            hash: "0xabc".to_string(),
            timestamp: "0x60000000".to_string(),
            gas_used: "0x5208".to_string(),
            gas_limit: "0x1c9c380".to_string(),
            base_fee_per_gas: Some("0x3b9aca00".to_string()),
            transactions: vec![],
            miner: "0x0000000000000000000000000000000000000000".to_string(),
        }
    }

    #[test]
    fn test_block_number_dec() {
        assert_eq!(make_block().number_dec(), 256);
    }

    #[test]
    fn test_block_timestamp_dec() {
        assert_eq!(make_block().timestamp_dec(), 0x60000000);
    }

    #[test]
    fn test_block_gas_used_dec() {
        assert_eq!(make_block().gas_used_dec(), 21000);
    }

    #[test]
    fn test_block_gas_limit_dec() {
        assert_eq!(make_block().gas_limit_dec(), 30_000_000);
    }

    #[test]
    fn test_block_base_fee_gwei() {
        let block = make_block();
        assert!((block.base_fee_gwei().unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_block_base_fee_none() {
        let mut block = make_block();
        block.base_fee_per_gas = None;
        assert!(block.base_fee_gwei().is_none());
    }

    #[test]
    fn test_block_gas_usage_percent() {
        let block = make_block();
        let pct = block.gas_usage_percent();
        let expected = (21000.0 / 30_000_000.0) * 100.0;
        assert!((pct - expected).abs() < 1e-6);
    }

    #[test]
    fn test_block_gas_usage_zero_limit() {
        let mut block = make_block();
        block.gas_limit = "0x0".to_string();
        assert_eq!(block.gas_usage_percent(), 0.0);
    }

    fn make_tx() -> Transaction {
        Transaction {
            hash: "0xabc".to_string(),
            block_number: Some("0x100".to_string()),
            from: "0xsender".to_string(),
            to: Some("0xreceiver".to_string()),
            value: "0xde0b6b3a7640000".to_string(),
            gas: "0x5208".to_string(),
            gas_price: Some("0x3b9aca00".to_string()),
            input: "0xa9059cbb0000".to_string(),
            nonce: "0xa".to_string(),
            tx_type: Some("0x2".to_string()),
        }
    }

    #[test]
    fn test_tx_value_ether() {
        assert!((make_tx().value_ether() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tx_gas_limit_dec() {
        assert_eq!(make_tx().gas_limit_dec(), 21000);
    }

    #[test]
    fn test_tx_gas_price_gwei() {
        assert!((make_tx().gas_price_gwei().unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tx_gas_price_none() {
        let mut tx = make_tx();
        tx.gas_price = None;
        assert!(tx.gas_price_gwei().is_none());
    }

    #[test]
    fn test_tx_nonce_dec() {
        assert_eq!(make_tx().nonce_dec(), 10);
    }

    #[test]
    fn test_tx_block_number_dec() {
        assert_eq!(make_tx().block_number_dec(), Some(256));
    }

    #[test]
    fn test_tx_block_number_pending() {
        let mut tx = make_tx();
        tx.block_number = None;
        assert_eq!(tx.block_number_dec(), None);
    }

    #[test]
    fn test_tx_input_preview_transfer() {
        let mut tx = make_tx();
        tx.input = "0x".to_string();
        assert_eq!(tx.input_preview(), "0x (transfer)");
    }

    #[test]
    fn test_tx_input_preview_empty() {
        let mut tx = make_tx();
        tx.input = String::new();
        assert_eq!(tx.input_preview(), "0x (transfer)");
    }

    #[test]
    fn test_tx_input_preview_method_selector() {
        let tx = make_tx();
        assert_eq!(tx.input_preview(), "0xa9059cbb");
    }

    #[test]
    fn test_tx_input_preview_short() {
        let mut tx = make_tx();
        tx.input = "0xa9059c".to_string();
        assert_eq!(tx.input_preview(), "0xa9059c");
    }

    fn make_receipt() -> TransactionReceipt {
        TransactionReceipt {
            status: Some("0x1".to_string()),
            gas_used: "0x5208".to_string(),
            effective_gas_price: Some("0x3b9aca00".to_string()),
            contract_address: None,
        }
    }

    #[test]
    fn test_receipt_succeeded_true() {
        assert_eq!(make_receipt().succeeded(), Some(true));
    }

    #[test]
    fn test_receipt_succeeded_false() {
        let mut r = make_receipt();
        r.status = Some("0x0".to_string());
        assert_eq!(r.succeeded(), Some(false));
    }

    #[test]
    fn test_receipt_succeeded_none() {
        let mut r = make_receipt();
        r.status = None;
        assert_eq!(r.succeeded(), None);
    }

    #[test]
    fn test_receipt_gas_used_dec() {
        assert_eq!(make_receipt().gas_used_dec(), 21000);
    }

    #[test]
    fn test_receipt_effective_gas_price_gwei() {
        assert!((make_receipt().effective_gas_price_gwei().unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_receipt_tx_cost_ether() {
        let r = make_receipt();
        let expected = (1_000_000_000u128 * 21000u128) as f64 / 1e18;
        assert!((r.tx_cost_ether() - expected).abs() < 1e-15);
    }

    #[test]
    fn test_receipt_tx_cost_no_gas_price() {
        let mut r = make_receipt();
        r.effective_gas_price = None;
        assert_eq!(r.tx_cost_ether(), 0.0);
    }
}
