use crate::config::{AppConfig, Network};
use crate::display;
use crate::rpc::RpcClient;

pub async fn execute(cfg: &AppConfig, aliases: Vec<String>, all: bool, rpc: Option<String>) {
    let networks = resolve_networks(cfg, aliases, all, rpc);

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

fn resolve_networks(
    cfg: &AppConfig,
    aliases: Vec<String>,
    all: bool,
    rpc: Option<String>,
) -> Vec<Network> {
    if all {
        return cfg.networks.clone();
    } else if !aliases.is_empty() {
        return aliases
            .into_iter()
            .map(|identifier| {
                if let Some(rpc_url) = &rpc {
                    Network {
                        name: identifier.clone(),
                        alias: identifier,
                        rpc_url: rpc_url.clone(),
                    }
                } else {
                    cfg.find_network(&identifier).cloned().unwrap_or_else(|| {
                        eprintln!("Network '{}' not found in config", identifier);
                        std::process::exit(1);
                    })
                }
            })
            .collect();
    }

    if let Some(default_alias) = &cfg.default_network {
        if let Some(net) = cfg.find_network(default_alias) {
            return vec![net.clone()];
        }
        eprintln!("Default network '{}' not found in config", default_alias);
        std::process::exit(1);
    }

    cfg.networks.clone()
}
