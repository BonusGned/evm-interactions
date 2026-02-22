use crate::model::Block;
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
