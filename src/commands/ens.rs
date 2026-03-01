use crate::config::{self, AppConfig};
use crate::display;
use crate::ens;
use crate::export::{self, OutputFormat};
use crate::rpc::RpcClient;

pub async fn execute(
    cfg: &AppConfig,
    name: String,
    alias: Option<String>,
    rpc: Option<String>,
    output: OutputFormat,
) {
    let network = config::resolve_network(cfg, alias, rpc);
    let client = RpcClient::new();

    match resolve(&client, &network.rpc_url, &name).await {
        Ok(address) => match output {
            OutputFormat::Table => {
                display::print_header();
                display::print_ens_result(&name, &address);
            }
            OutputFormat::Json => println!("{}", export::ens_to_json(&name, &address)),
            OutputFormat::Csv => println!("{name},{address}"),
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

async fn resolve(client: &RpcClient, rpc_url: &str, name: &str) -> Result<String, String> {
    let resolver_data = ens::encode_resolver_call(name);
    let resolver_result = client
        .eth_call(rpc_url, ens::ENS_REGISTRY, &resolver_data, "latest")
        .await?;

    let resolver_addr = ens::parse_address_from_result(&resolver_result)
        .ok_or_else(|| format!("no resolver found for '{name}'"))?;

    let addr_data = ens::encode_addr_call(name);
    let addr_result = client
        .eth_call(rpc_url, &resolver_addr, &addr_data, "latest")
        .await?;

    ens::parse_address_from_result(&addr_result)
        .ok_or_else(|| format!("name '{name}' has no address set"))
}
