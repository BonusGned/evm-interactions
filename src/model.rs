use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse {
    pub result: Option<Block>,
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
        u64::from_str_radix(self.number.trim_start_matches("0x"), 16).unwrap_or(0)
    }

    pub fn timestamp_dec(&self) -> i64 {
        i64::from_str_radix(self.timestamp.trim_start_matches("0x"), 16).unwrap_or(0)
    }

    pub fn gas_used_dec(&self) -> u64 {
        u64::from_str_radix(self.gas_used.trim_start_matches("0x"), 16).unwrap_or(0)
    }

    pub fn gas_limit_dec(&self) -> u64 {
        u64::from_str_radix(self.gas_limit.trim_start_matches("0x"), 16).unwrap_or(0)
    }

    pub fn base_fee_gwei(&self) -> Option<f64> {
        self.base_fee_per_gas.as_ref().map(|hex| {
            let wei = u128::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0);
            wei as f64 / 1_000_000_000.0
        })
    }

    pub fn gas_usage_percent(&self) -> f64 {
        let limit = self.gas_limit_dec();
        if limit == 0 {
            return 0.0;
        }
        (self.gas_used_dec() as f64 / limit as f64) * 100.0
    }
}
