use crate::config::{self, AppConfig};
use crate::display;
use crate::export::{self, OutputFormat};
use crate::model;
use crate::rpc::RpcClient;

pub async fn execute(
    cfg: &AppConfig,
    address: String,
    aliases: Vec<String>,
    all: bool,
    rpc: Option<String>,
    output: OutputFormat,
) {
    let networks = config::resolve_networks(cfg, aliases, all, rpc);

    if networks.is_empty() {
        eprintln!("No networks to query. Run `evm-interactions config init` to set up defaults.");
        std::process::exit(1);
    }

    if output == OutputFormat::Table {
        display::print_header();
    } else if output == OutputFormat::Csv {
        println!("{}", export::balance_csv_header());
    }

    let client = RpcClient::new();

    let futures: Vec<_> = networks
        .iter()
        .map(|net| {
            let client = client.clone();
            let url = net.rpc_url.clone();
            let addr = address.clone();
            async move { client.get_balance(&url, &addr).await }
        })
        .collect();

    let results = futures::future::join_all(futures).await;

    for (network, result) in networks.iter().zip(results) {
        match result {
            Ok(hex) => {
                let balance_eth = model::wei_hex_to_ether(&hex);
                match output {
                    OutputFormat::Table => display::print_balance(&network.name, &address, &hex),
                    OutputFormat::Json => {
                        println!(
                            "{}",
                            export::balance_to_json(&network.name, &address, balance_eth)
                        );
                    }
                    OutputFormat::Csv => {
                        println!(
                            "{}",
                            export::balance_to_csv(&network.name, &address, balance_eth)
                        );
                    }
                }
            }
            Err(err) => match output {
                OutputFormat::Table => display::print_error(&network.name, &err),
                _ => eprintln!("Error [{}]: {err}", network.name),
            },
        }
    }
}
