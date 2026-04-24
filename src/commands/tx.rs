use crate::config::{self, AppConfig};
use crate::display;
use crate::export::{self, OutputFormat};
use crate::rpc::RpcClient;

pub async fn execute(
    cfg: &AppConfig,
    hash: String,
    alias: Option<String>,
    rpc: Option<String>,
    output: OutputFormat,
) {
    let network = config::resolve_network(cfg, alias, rpc);
    let client = RpcClient::new();
    let url = &network.rpc_url;

    let (tx_result, receipt_result) = tokio::join!(
        client.get_transaction(url, &hash),
        client.get_transaction_receipt(url, &hash),
    );

    match tx_result {
        Ok(tx) => {
            let receipt = receipt_result.ok();
            match output {
                OutputFormat::Table => {
                    display::print_header();
                    display::print_transaction(&network.name, &tx, receipt.as_ref());
                }
                OutputFormat::Json => {
                    println!(
                        "{}",
                        export::tx_to_json(&network.name, &tx, receipt.as_ref())
                    );
                }
                OutputFormat::Csv => {
                    println!("{}", export::tx_csv_header());
                    println!("{}", export::tx_to_csv(&network.name, &tx));
                }
            }
        }
        Err(err) => {
            let msg = if err == "empty result" {
                "transaction not found"
            } else {
                &err
            };
            match output {
                OutputFormat::Table => {
                    display::print_header();
                    display::print_error(&network.name, msg);
                }
                _ => eprintln!("Error: {msg}"),
            }
        }
    }
}
