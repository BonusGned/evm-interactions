use crate::config::{self, AppConfig};
use crate::display;
use crate::rpc::RpcClient;

pub async fn execute(cfg: &AppConfig, aliases: Vec<String>, all: bool, rpc: Option<String>) {
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
            async move {
                let price = client.get_gas_price(&url).await;
                let priority = client.get_max_priority_fee(&url).await.ok();
                (price, priority)
            }
        })
        .collect();

    let results = futures::future::join_all(futures).await;

    for (network, (price_result, priority_fee)) in networks.iter().zip(results) {
        match price_result {
            Ok(hex) => display::print_gas(&network.name, &hex, priority_fee.as_deref()),
            Err(err) => display::print_error(&network.name, &err),
        }
    }
}
