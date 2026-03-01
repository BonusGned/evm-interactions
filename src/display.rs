use crate::model::{self, Block, Log, Transaction, TransactionReceipt};
use chrono::{DateTime, Utc};
use colored::Colorize;

const SEPARATOR: &str = "──────────────────────────────────────────────────────────";
const BORDER: &str = "══════════════════════════════════════════════════════════";

pub fn print_header() {
    println!("\n{}", BORDER.bright_cyan());
    println!(
        "{}",
        "            ⛓  evm-interactions · EVM explorer  ⛓"
            .bright_white()
            .bold()
    );
    println!("{}\n", BORDER.bright_cyan());
}

pub fn print_block(network_name: &str, block: &Block) {
    let timestamp = DateTime::<Utc>::from_timestamp(block.timestamp_dec(), 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let base_fee = block
        .base_fee_gwei()
        .map(|g| format!("{g:.4} Gwei"))
        .unwrap_or_else(|| "N/A".to_string());

    println!(
        "  {} {}",
        "Network:".bright_yellow(),
        network_name.bright_white().bold()
    );
    println!(
        "  {} #{}",
        "Block:".bright_yellow(),
        block.number_dec().to_string().bright_green()
    );
    println!("  {} {}", "Hash:".bright_yellow(), block.hash.dimmed());
    println!("  {} {}", "Time:".bright_yellow(), timestamp);
    println!("  {} {}", "Miner:".bright_yellow(), block.miner.dimmed());
    println!(
        "  {} {} / {} ({:.1}%)",
        "Gas:".bright_yellow(),
        format_number(block.gas_used_dec()),
        format_number(block.gas_limit_dec()),
        block.gas_usage_percent()
    );
    println!("  {} {}", "Base Fee:".bright_yellow(), base_fee);
    println!(
        "  {} {}",
        "Txns:".bright_yellow(),
        block.transactions.len().to_string().bright_cyan()
    );
    println!("  {}", SEPARATOR.dimmed());
}

pub fn print_transaction(
    network_name: &str,
    tx: &Transaction,
    receipt: Option<&TransactionReceipt>,
) {
    let block = tx
        .block_number_dec()
        .map(|n| format!("#{}", format_number(n)))
        .unwrap_or_else(|| "pending".to_string());

    let status = receipt
        .and_then(|r| r.succeeded())
        .map(|ok| {
            if ok {
                "Success ✓".bright_green().to_string()
            } else {
                "Failed ✗".bright_red().to_string()
            }
        })
        .unwrap_or_else(|| "Pending".bright_yellow().to_string());

    let to = tx.to.as_deref().unwrap_or("Contract Creation");

    let gas_info = receipt
        .map(|r| {
            let used = r.gas_used_dec();
            let limit = tx.gas_limit_dec();
            let pct = if limit > 0 {
                (used as f64 / limit as f64) * 100.0
            } else {
                0.0
            };
            format!(
                "{} / {} ({:.1}%)",
                format_number(used),
                format_number(limit),
                pct
            )
        })
        .unwrap_or_else(|| format!("{} (limit)", format_number(tx.gas_limit_dec())));

    let gas_price = receipt
        .and_then(|r| r.effective_gas_price_gwei())
        .or(tx.gas_price_gwei())
        .map(|g| format!("{g:.4} Gwei"))
        .unwrap_or_else(|| "N/A".to_string());

    let cost = receipt
        .map(|r| format_ether(r.tx_cost_ether()))
        .unwrap_or_else(|| "N/A".to_string());

    println!(
        "  {} {}",
        "Network:".bright_yellow(),
        network_name.bright_white().bold()
    );
    println!("  {} {}", "Tx Hash:".bright_yellow(), tx.hash.dimmed());
    println!("  {} {}", "Status:".bright_yellow(), status);
    println!("  {} {}", "Block:".bright_yellow(), block.bright_green());
    println!("  {} {}", "From:".bright_yellow(), tx.from.dimmed());
    println!("  {} {}", "To:".bright_yellow(), to.dimmed());
    println!(
        "  {} {} ETH",
        "Value:".bright_yellow(),
        format_ether(tx.value_ether())
    );
    println!("  {} {}", "Gas:".bright_yellow(), gas_info);
    println!("  {} {}", "Gas Price:".bright_yellow(), gas_price);
    println!("  {} {} ETH", "Tx Cost:".bright_yellow(), cost);
    println!("  {} {}", "Nonce:".bright_yellow(), tx.nonce_dec());
    println!(
        "  {} {}",
        "Input:".bright_yellow(),
        tx.input_preview().dimmed()
    );

    if let Some(addr) = receipt.and_then(|r| r.contract_address.as_deref()) {
        println!("  {} {}", "Contract:".bright_yellow(), addr.bright_cyan());
    }

    println!("  {}", SEPARATOR.dimmed());
}

pub fn print_balance(network_name: &str, address: &str, balance_hex: &str) {
    let balance = model::wei_hex_to_ether(balance_hex);

    println!(
        "  {} {}",
        "Network:".bright_yellow(),
        network_name.bright_white().bold()
    );
    println!("  {} {}", "Address:".bright_yellow(), address.dimmed());
    println!(
        "  {} {} ETH",
        "Balance:".bright_yellow(),
        format_ether(balance).bright_green()
    );
    println!("  {}", SEPARATOR.dimmed());
}

pub fn print_gas(network_name: &str, gas_price_hex: &str, priority_fee_hex: Option<&str>) {
    let gas_price = model::wei_hex_to_gwei(gas_price_hex);

    println!(
        "  {} {}",
        "Network:".bright_yellow(),
        network_name.bright_white().bold()
    );
    println!("  {} {:.4} Gwei", "Gas Price:".bright_yellow(), gas_price);

    if let Some(hex) = priority_fee_hex {
        let priority = model::wei_hex_to_gwei(hex);
        println!("  {} {:.4} Gwei", "Priority:".bright_yellow(), priority);
        let base = gas_price - priority;
        if base > 0.0 {
            println!("  {} {:.4} Gwei", "Base Fee:".bright_yellow(), base);
        }
    }

    println!("  {}", SEPARATOR.dimmed());
}

pub fn print_call_result(network_name: &str, address: &str, data: &str, result: &str) {
    let data_preview = if data.len() > 10 { &data[..10] } else { data };

    println!(
        "  {} {}",
        "Network:".bright_yellow(),
        network_name.bright_white().bold()
    );
    println!("  {} {}", "Contract:".bright_yellow(), address.dimmed());
    println!(
        "  {} {}...",
        "Calldata:".bright_yellow(),
        data_preview.dimmed()
    );
    println!("  {} {}", "Result:".bright_yellow(), result.bright_green());
    println!("  {}", SEPARATOR.dimmed());
}

pub fn print_log(network_name: &str, log: &Log) {
    let block = log
        .block_number_dec()
        .map(|n| format!("#{}", format_number(n)))
        .unwrap_or_else(|| "pending".to_string());

    let index = log
        .log_index_dec()
        .map(|n| n.to_string())
        .unwrap_or_else(|| "?".to_string());

    println!(
        "  {} {}",
        "Network:".bright_yellow(),
        network_name.bright_white().bold()
    );
    println!(
        "  {} {}  {} {}",
        "Block:".bright_yellow(),
        block.bright_green(),
        "Index:".bright_yellow(),
        index
    );
    println!("  {} {}", "Address:".bright_yellow(), log.address.dimmed());

    for (i, topic) in log.topics.iter().enumerate() {
        println!(
            "  {} {}",
            format!("Topic[{i}]:").bright_yellow(),
            topic.dimmed()
        );
    }

    println!(
        "  {} {}",
        "Data:".bright_yellow(),
        log.data_preview().dimmed()
    );

    if let Some(tx) = &log.transaction_hash {
        println!("  {} {}", "Tx:".bright_yellow(), tx.dimmed());
    }

    println!("  {}", SEPARATOR.dimmed());
}

pub fn print_logs_summary(count: usize) {
    println!(
        "  {} {} event(s) found",
        "Total:".bright_yellow(),
        count.to_string().bright_cyan()
    );
}

pub fn print_ens_result(name: &str, address: &str) {
    println!(
        "  {} {}",
        "Name:".bright_yellow(),
        name.bright_white().bold()
    );
    println!(
        "  {} {}",
        "Address:".bright_yellow(),
        address.bright_green()
    );
    println!("  {}", SEPARATOR.dimmed());
}

pub fn print_error(network_name: &str, error: &str) {
    println!(
        "  {} {} — {}",
        "✗".bright_red(),
        network_name.bright_white().bold(),
        error.red()
    );
    println!("  {}", SEPARATOR.dimmed());
}

pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn format_ether(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else if value < 0.0001 {
        let s = format!("{value:.10}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        let s = format!("{value:.6}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number_zero() {
        assert_eq!(format_number(0), "0");
    }

    #[test]
    fn test_format_number_small() {
        assert_eq!(format_number(1), "1");
        assert_eq!(format_number(999), "999");
    }

    #[test]
    fn test_format_number_thousands() {
        assert_eq!(format_number(1_000), "1,000");
        assert_eq!(format_number(1_234_567), "1,234,567");
        assert_eq!(format_number(30_000_000), "30,000,000");
    }

    #[test]
    fn test_format_ether_zero() {
        assert_eq!(format_ether(0.0), "0");
    }

    #[test]
    fn test_format_ether_small_value() {
        assert_eq!(format_ether(0.00001), "0.00001");
    }

    #[test]
    fn test_format_ether_normal_value() {
        assert_eq!(format_ether(1.5), "1.5");
        assert_eq!(format_ether(1.0), "1");
    }

    #[test]
    fn test_format_ether_precise() {
        assert_eq!(format_ether(1.123456), "1.123456");
    }
}
