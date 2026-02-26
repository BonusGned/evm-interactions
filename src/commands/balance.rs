use crate::config::{self, AppConfig};
use crate::display;
use crate::rpc::RpcClient;

pub async fn execute(
    cfg: &AppConfig,
    address: String,
    aliases: Vec<String>,
    all: bool,
    rpc: Option<String>,
) {
    let networks = config::resolve_networks(cfg, aliases, all, rpc);

    if networks.is_empty() {
        eprintln!("No networks to query. Run `evm-interactions config init` to set up defaults.");
        std::process::exit(1);
    }

    display::print_header();

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
            Ok(hex) => display::print_balance(&network.name, &address, &hex),
            Err(err) => display::print_error(&network.name, &err),
        }
    }
}
