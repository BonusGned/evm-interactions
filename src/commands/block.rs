use crate::config::{self, AppConfig};
use crate::display;
use crate::rpc::RpcClient;

pub async fn execute(
    cfg: &AppConfig,
    aliases: Vec<String>,
    all: bool,
    rpc: Option<String>,
    number: Option<String>,
) {
    let networks = config::resolve_networks(cfg, aliases, all, rpc);

    if networks.is_empty() {
        eprintln!("No networks to query. Run `evm-interactions config init` to set up defaults.");
        std::process::exit(1);
    }

    let block_id = number
        .map(|n| parse_block_number(&n))
        .unwrap_or_else(|| "latest".to_string());

    display::print_header();

    let client = RpcClient::new();

    let futures: Vec<_> = networks
        .iter()
        .map(|net| {
            let client = client.clone();
            let url = net.rpc_url.clone();
            let bid = block_id.clone();
            async move { client.get_block(&url, &bid).await }
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

fn parse_block_number(input: &str) -> String {
    if input.starts_with("0x") || matches!(input, "latest" | "earliest" | "pending") {
        return input.to_string();
    }
    match input.parse::<u64>() {
        Ok(num) => format!("0x{num:x}"),
        Err(_) => input.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_block_number_hex() {
        assert_eq!(parse_block_number("0x123"), "0x123");
        assert_eq!(parse_block_number("0xff"), "0xff");
    }

    #[test]
    fn test_parse_block_number_decimal() {
        assert_eq!(parse_block_number("123"), "0x7b");
        assert_eq!(parse_block_number("0"), "0x0");
        assert_eq!(parse_block_number("1000000"), "0xf4240");
    }

    #[test]
    fn test_parse_block_number_keywords() {
        assert_eq!(parse_block_number("latest"), "latest");
        assert_eq!(parse_block_number("earliest"), "earliest");
        assert_eq!(parse_block_number("pending"), "pending");
    }

    #[test]
    fn test_parse_block_number_invalid() {
        assert_eq!(parse_block_number("abc"), "abc");
        assert_eq!(parse_block_number("not_a_number"), "not_a_number");
    }
}
