mod config;
mod display;
mod model;
mod rpc;

use clap::Parser;
use config::{default_networks, Network};
use rpc::RpcClient;

#[derive(Parser)]
#[command(name = "latest-block", about = "Fetch latest blocks from EVM networks")]
struct Cli {
    /// Custom network name
    #[arg(long)]
    name: Option<String>,

    /// Custom network RPC URL
    #[arg(long)]
    rpc: Option<String>,

    /// Skip default networks, only query custom
    #[arg(long, default_value_t = false)]
    custom_only: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = RpcClient::new();

    let mut networks: Vec<Network> = if cli.custom_only {
        Vec::new()
    } else {
        default_networks()
    };

    if let (Some(name), Some(rpc_url)) = (&cli.name, &cli.rpc) {
        networks.push(Network::new(name, rpc_url));
    } else if cli.name.is_some() || cli.rpc.is_some() {
        eprintln!("Both --name and --rpc are required for a custom network");
        std::process::exit(1);
    }

    if networks.is_empty() {
        eprintln!("No networks to query");
        std::process::exit(1);
    }

    display::print_header();

    let futures: Vec<_> = networks
        .iter()
        .map(|net| {
            let client = client.clone();
            let url = net.rpc_url.clone();
            async move { client.get_latest_block(&url).await }
        })
        .collect();

    let results = futures::future::join_all(futures).await;

    for (network, result) in networks.iter().zip(results) {
        match result {
            Ok(block) => display::print_block(&network.name, &block),
            Err(err) => display::print_error(&network.name, &err),
        }
    }
}
