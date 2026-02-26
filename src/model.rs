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
