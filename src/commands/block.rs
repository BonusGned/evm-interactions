use crate::config::{self, AppConfig};
use crate::display;
use crate::export::{self, OutputFormat};
use crate::rpc::RpcClient;

#[allow(clippy::too_many_arguments)]
pub async fn execute(
    cfg: &AppConfig,
    aliases: Vec<String>,
    all: bool,
    rpc: Option<String>,
    number: Option<String>,
    watch: bool,
    interval: u64,
    output: OutputFormat,
) {
    let networks = config::resolve_networks(cfg, aliases, all, rpc);

    if networks.is_empty() {
        eprintln!("No networks to query. Run `evm-interactions config init` to set up defaults.");
        std::process::exit(1);
    }

    let block_id = number
        .map(|n| parse_block_number(&n))
        .unwrap_or_else(|| "latest".to_string());

    let client = RpcClient::new();

    if watch {
        run_watch(&client, &networks, interval, output).await;
    } else {
        fetch_once(&client, &networks, &block_id, output).await;
    }
}

async fn fetch_once(
    client: &RpcClient,
    networks: &[crate::config::Network],
    block_id: &str,
    output: OutputFormat,
) {
    if output == OutputFormat::Table {
        display::print_header();
    } else if output == OutputFormat::Csv {
        println!("{}", export::block_csv_header());
    }

    let futures: Vec<_> = networks
        .iter()
        .map(|net| {
            let client = client.clone();
            let url = net.rpc_url.clone();
            let bid = block_id.to_string();
            async move { client.get_block(&url, &bid).await }
        })
        .collect();

    let results = futures::future::join_all(futures).await;

    for (network, result) in networks.iter().zip(results) {
        match result {
            Ok(block) => match output {
                OutputFormat::Table => display::print_block(&network.name, &block),
                OutputFormat::Json => println!("{}", export::block_to_json(&network.name, &block)),
                OutputFormat::Csv => println!("{}", export::block_to_csv(&network.name, &block)),
            },
            Err(err) => match output {
                OutputFormat::Table => display::print_error(&network.name, &err),
                _ => eprintln!("Error [{}]: {err}", network.name),
            },
        }
    }
}

async fn run_watch(
    client: &RpcClient,
    networks: &[crate::config::Network],
    interval: u64,
    output: OutputFormat,
) {
    let mut last_blocks: Vec<Option<u64>> = vec![None; networks.len()];

    if output == OutputFormat::Table {
        display::print_header();
        println!(
            "  Watching {} network(s) every {}s... (Ctrl+C to stop)\n",
            networks.len(),
            interval
        );
    }

    loop {
        let futures: Vec<_> = networks
            .iter()
            .map(|net| {
                let client = client.clone();
                let url = net.rpc_url.clone();
                async move { client.get_block(&url, "latest").await }
            })
            .collect();

        let results = futures::future::join_all(futures).await;

        for (i, (network, result)) in networks.iter().zip(results).enumerate() {
            match result {
                Ok(block) => {
                    let num = block.number_dec();
                    if last_blocks[i] != Some(num) {
                        last_blocks[i] = Some(num);
                        match output {
                            OutputFormat::Table => display::print_block(&network.name, &block),
                            OutputFormat::Json => {
                                println!("{}", export::block_to_json(&network.name, &block))
                            }
                            OutputFormat::Csv => {
                                println!("{}", export::block_to_csv(&network.name, &block))
                            }
                        }
                    }
                }
                Err(err) => match output {
                    OutputFormat::Table => display::print_error(&network.name, &err),
                    _ => eprintln!("Error [{}]: {err}", network.name),
                },
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
    }
}

pub fn parse_block_number(input: &str) -> String {
    if input.starts_with("0x")
        || matches!(
            input,
            "latest" | "earliest" | "pending" | "safe" | "finalized"
        )
    {
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
    fn test_parse_block_number_safe_finalized() {
        assert_eq!(parse_block_number("safe"), "safe");
        assert_eq!(parse_block_number("finalized"), "finalized");
    }

    #[test]
    fn test_parse_block_number_invalid() {
        assert_eq!(parse_block_number("abc"), "abc");
        assert_eq!(parse_block_number("not_a_number"), "not_a_number");
    }
}
