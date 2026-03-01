use crate::config::{self, AppConfig};
use crate::display;
use crate::export::{self, OutputFormat};
use crate::rpc::RpcClient;

#[allow(clippy::too_many_arguments)]
pub async fn execute(
    cfg: &AppConfig,
    address: String,
    topics: Vec<String>,
    from: String,
    to: String,
    alias: Option<String>,
    rpc: Option<String>,
    output: OutputFormat,
) {
    let network = config::resolve_network(cfg, alias, rpc);
    let client = RpcClient::new();

    let from_block = super::block::parse_block_number(&from);
    let to_block = super::block::parse_block_number(&to);

    match client
        .get_logs(&network.rpc_url, &address, &topics, &from_block, &to_block)
        .await
    {
        Ok(logs) => match output {
            OutputFormat::Table => {
                display::print_header();
                if logs.is_empty() {
                    println!("  No logs found for {address}");
                } else {
                    for log in &logs {
                        display::print_log(&network.name, log);
                    }
                    display::print_logs_summary(logs.len());
                }
            }
            OutputFormat::Json => {
                for log in &logs {
                    println!("{}", export::log_to_json(&network.name, log));
                }
            }
            OutputFormat::Csv => {
                println!("{}", export::log_csv_header());
                for log in &logs {
                    println!("{}", export::log_to_csv(&network.name, log));
                }
            }
        },
        Err(err) => match output {
            OutputFormat::Table => {
                display::print_header();
                display::print_error(&network.name, &err);
            }
            _ => eprintln!("Error: {err}"),
        },
    }
}
