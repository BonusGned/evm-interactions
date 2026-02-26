use crate::config::{self, AppConfig};
use crate::display;
use crate::rpc::RpcClient;

pub async fn execute(cfg: &AppConfig, hash: String, alias: Option<String>, rpc: Option<String>) {
    let network = config::resolve_network(cfg, alias, rpc);

    display::print_header();

    let client = RpcClient::new();
    let url = &network.rpc_url;

    let (tx_result, receipt_result) = tokio::join!(
        client.get_transaction(url, &hash),
        client.get_transaction_receipt(url, &hash),
    );

    match tx_result {
        Ok(tx) => display::print_transaction(&network.name, &tx, receipt_result.ok().as_ref()),
        Err(err) => {
            let msg = if err == "empty result" {
                "transaction not found"
            } else {
                &err
            };
            display::print_error(&network.name, msg);
        }
    }
}
