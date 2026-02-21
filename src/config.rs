pub struct Network {
    pub name: String,
    pub rpc_url: String,
}

impl Network {
    pub fn new(name: impl Into<String>, rpc_url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rpc_url: rpc_url.into(),
        }
    }
}

pub fn default_networks() -> Vec<Network> {
    vec![
        Network::new("Ethereum", "https://ethereum-rpc.publicnode.com"),
        Network::new("BSC", "https://bsc-dataseed1.binance.org"),
        Network::new("Polygon", "https://polygon-bor-rpc.publicnode.com"),
        Network::new("Avalanche C-Chain", "https://api.avax.network/ext/bc/C/rpc"),
        Network::new("Sonic", "https://rpc.soniclabs.com"),
    ]
}
