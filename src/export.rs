use crate::model::{Block, Log, Transaction, TransactionReceipt};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            other => Err(format!("unknown output format: '{other}'")),
        }
    }
}

pub fn block_to_json(network: &str, block: &Block) -> String {
    let obj = serde_json::json!({
        "network": network,
        "number": block.number_dec(),
        "hash": block.hash,
        "timestamp": block.timestamp_dec(),
        "miner": block.miner,
        "gasUsed": block.gas_used_dec(),
        "gasLimit": block.gas_limit_dec(),
        "baseFeeGwei": block.base_fee_gwei(),
        "txCount": block.transactions.len(),
    });
    serde_json::to_string(&obj).unwrap_or_default()
}

pub fn tx_to_json(network: &str, tx: &Transaction, receipt: Option<&TransactionReceipt>) -> String {
    let status = receipt
        .and_then(|r| r.succeeded())
        .map(|ok| if ok { "success" } else { "failed" });
    let obj = serde_json::json!({
        "network": network,
        "hash": tx.hash,
        "status": status,
        "blockNumber": tx.block_number_dec(),
        "from": tx.from,
        "to": tx.to,
        "valueEth": tx.value_ether(),
        "gasLimit": tx.gas_limit_dec(),
        "gasPriceGwei": tx.gas_price_gwei(),
        "nonce": tx.nonce_dec(),
        "input": tx.input_preview(),
        "gasUsed": receipt.map(|r| r.gas_used_dec()),
        "txCostEth": receipt.map(|r| r.tx_cost_ether()),
    });
    serde_json::to_string(&obj).unwrap_or_default()
}

pub fn balance_to_json(network: &str, address: &str, balance_eth: f64) -> String {
    let obj = serde_json::json!({
        "network": network,
        "address": address,
        "balanceEth": balance_eth,
    });
    serde_json::to_string(&obj).unwrap_or_default()
}

pub fn gas_to_json(network: &str, gas_gwei: f64, priority_gwei: Option<f64>) -> String {
    let obj = serde_json::json!({
        "network": network,
        "gasPriceGwei": gas_gwei,
        "priorityFeeGwei": priority_gwei,
        "baseFeeGwei": priority_gwei.map(|p| gas_gwei - p),
    });
    serde_json::to_string(&obj).unwrap_or_default()
}

pub fn call_to_json(network: &str, address: &str, result: &str) -> String {
    let obj = serde_json::json!({
        "network": network,
        "address": address,
        "result": result,
    });
    serde_json::to_string(&obj).unwrap_or_default()
}

pub fn log_to_json(network: &str, log: &Log) -> String {
    let obj = serde_json::json!({
        "network": network,
        "address": log.address,
        "topics": log.topics,
        "data": log.data,
        "blockNumber": log.block_number_dec(),
        "transactionHash": log.transaction_hash,
        "logIndex": log.log_index_dec(),
    });
    serde_json::to_string(&obj).unwrap_or_default()
}

pub fn ens_to_json(name: &str, address: &str) -> String {
    let obj = serde_json::json!({
        "name": name,
        "address": address,
    });
    serde_json::to_string(&obj).unwrap_or_default()
}

pub fn block_csv_header() -> &'static str {
    "network,number,hash,timestamp,miner,gasUsed,gasLimit,baseFeeGwei,txCount"
}

pub fn block_to_csv(network: &str, block: &Block) -> String {
    format!(
        "{},{},{},{},{},{},{},{},{}",
        network,
        block.number_dec(),
        block.hash,
        block.timestamp_dec(),
        block.miner,
        block.gas_used_dec(),
        block.gas_limit_dec(),
        block
            .base_fee_gwei()
            .map(|g| format!("{g:.4}"))
            .unwrap_or_default(),
        block.transactions.len(),
    )
}

pub fn balance_csv_header() -> &'static str {
    "network,address,balanceEth"
}

pub fn balance_to_csv(network: &str, address: &str, balance_eth: f64) -> String {
    format!("{network},{address},{balance_eth}")
}

pub fn gas_csv_header() -> &'static str {
    "network,gasPriceGwei,priorityFeeGwei,baseFeeGwei"
}

pub fn gas_to_csv(network: &str, gas_gwei: f64, priority_gwei: Option<f64>) -> String {
    let priority_str = priority_gwei.map(|p| format!("{p:.4}")).unwrap_or_default();
    let base_str = priority_gwei
        .map(|p| format!("{:.4}", gas_gwei - p))
        .unwrap_or_default();
    format!("{network},{gas_gwei:.4},{priority_str},{base_str}")
}

pub fn log_csv_header() -> &'static str {
    "network,address,blockNumber,txHash,logIndex,topic0,data"
}

pub fn log_to_csv(network: &str, log: &Log) -> String {
    let topic0 = log.topics.first().map(|t| t.as_str()).unwrap_or("");
    format!(
        "{},{},{},{},{},{},{}",
        network,
        log.address,
        log.block_number_dec()
            .map(|n| n.to_string())
            .unwrap_or_default(),
        log.transaction_hash.as_deref().unwrap_or(""),
        log.log_index_dec()
            .map(|n| n.to_string())
            .unwrap_or_default(),
        topic0,
        log.data_preview(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn make_tx() -> Transaction {
        Transaction {
            hash: "0xabc".to_string(),
            block_number: Some("0x100".to_string()),
            from: "0xsender".to_string(),
            to: Some("0xreceiver".to_string()),
            value: "0xde0b6b3a7640000".to_string(),
            gas: "0x5208".to_string(),
            gas_price: Some("0x3b9aca00".to_string()),
            input: "0x".to_string(),
            nonce: "0xa".to_string(),
            tx_type: Some("0x2".to_string()),
        }
    }

    #[test]
    fn test_output_format_parse() {
        assert_eq!(
            "table".parse::<OutputFormat>().unwrap(),
            OutputFormat::Table
        );
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("csv".parse::<OutputFormat>().unwrap(), OutputFormat::Csv);
        assert_eq!("JSON".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert!("xml".parse::<OutputFormat>().is_err());
    }

    #[test]
    fn test_block_to_json() {
        let json = block_to_json("Ethereum", &make_block());
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["network"], "Ethereum");
        assert_eq!(parsed["number"], 256);
        assert_eq!(parsed["hash"], "0xabc");
    }

    #[test]
    fn test_tx_to_json() {
        let json = tx_to_json("Ethereum", &make_tx(), None);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["network"], "Ethereum");
        assert_eq!(parsed["hash"], "0xabc");
        assert!(parsed["status"].is_null());
    }

    #[test]
    fn test_balance_to_json() {
        let json = balance_to_json("Ethereum", "0xaddr", 1.5);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["network"], "Ethereum");
        assert_eq!(parsed["address"], "0xaddr");
        assert_eq!(parsed["balanceEth"], 1.5);
    }

    #[test]
    fn test_block_to_csv() {
        let csv = block_to_csv("Ethereum", &make_block());
        assert!(csv.starts_with("Ethereum,256,"));
        assert!(csv.contains("0xabc"));
    }

    #[test]
    fn test_balance_to_csv() {
        let csv = balance_to_csv("Ethereum", "0xaddr", 1.5);
        assert_eq!(csv, "Ethereum,0xaddr,1.5");
    }

    #[test]
    fn test_gas_to_json() {
        let json = gas_to_json("Ethereum", 15.5, Some(1.0));
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["gasPriceGwei"], 15.5);
        assert_eq!(parsed["priorityFeeGwei"], 1.0);
        assert_eq!(parsed["baseFeeGwei"], 14.5);
    }

    #[test]
    fn test_log_to_json() {
        let log = crate::model::Log {
            address: "0xcontract".to_string(),
            topics: vec!["0xtopic".to_string()],
            data: "0xdata".to_string(),
            block_number: Some("0x100".to_string()),
            transaction_hash: Some("0xtxhash".to_string()),
            log_index: Some("0x0".to_string()),
            transaction_index: Some("0x0".to_string()),
        };
        let json = log_to_json("Ethereum", &log);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["address"], "0xcontract");
        assert_eq!(parsed["blockNumber"], 256);
    }

    #[test]
    fn test_ens_to_json() {
        let json = ens_to_json("vitalik.eth", "0xd8da6bf26964af9d7eed9e03e53415d37aa96045");
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["name"], "vitalik.eth");
        assert!(parsed["address"].as_str().unwrap().starts_with("0x"));
    }
}
