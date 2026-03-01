use crate::config::{self, AppConfig};
use crate::display;
use crate::export::{self, OutputFormat};
use crate::rpc::RpcClient;

pub async fn execute(
    cfg: &AppConfig,
    address: String,
    data: String,
    alias: Option<String>,
    rpc: Option<String>,
    block: String,
    output: OutputFormat,
) {
    let network = config::resolve_network(cfg, alias, rpc);
    let client = RpcClient::new();

    let block_tag = super::block::parse_block_number(&block);

    match client
        .eth_call(&network.rpc_url, &address, &data, &block_tag)
        .await
    {
        Ok(result) => match output {
            OutputFormat::Table => {
                display::print_header();
                display::print_call_result(&network.name, &address, &data, &result);
            }
            OutputFormat::Json => {
                println!("{}", export::call_to_json(&network.name, &address, &result))
            }
            OutputFormat::Csv => println!("{}", result),
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
