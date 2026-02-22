use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    pub alias: String,
    pub rpc_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub default_network: Option<String>,
    #[serde(default)]
    pub networks: Vec<Network>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_network: Some("eth".to_string()),
            networks: default_networks(),
        }
    }
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("failed to read config: {e}"))?;
        toml::from_str(&content).map_err(|e| format!("failed to parse config: {e}"))
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create config directory: {e}"))?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| format!("failed to serialize config: {e}"))?;
        std::fs::write(path, content).map_err(|e| format!("failed to write config: {e}"))
    }

    pub fn find_network(&self, identifier: &str) -> Option<&Network> {
        let lower = identifier.to_lowercase();
        self.networks.iter().find(|n| n.name.to_lowercase() == lower || n.alias.to_lowercase() == lower)
    }

    pub fn remove_network(&mut self, identifier: &str) -> bool {
        let lower = identifier.to_lowercase();
        let before = self.networks.len();
        self.networks.retain(|n| n.name.to_lowercase() != lower && n.alias.to_lowercase() != lower);
        self.networks.len() < before
    }

    pub fn add_network(&mut self, name: String, alias: String, rpc_url: String) -> Result<(), String> {
        if self.find_network(&alias).is_some() || self.find_network(&name).is_some() {
            return Err(format!("network with name '{}' or alias '{}' already exists", name, alias));
        }
        self.networks.push(Network { name, alias, rpc_url });
        Ok(())
    }
}

pub fn default_networks() -> Vec<Network> {
    vec![
        Network { name: "Ethereum".into(), alias: "eth".into(), rpc_url: "https://ethereum-rpc.publicnode.com".into() },
        Network { name: "BSC".into(), alias: "bsc".into(), rpc_url: "https://bsc-dataseed1.binance.org".into() },
        Network { name: "Polygon".into(), alias: "matic".into(), rpc_url: "https://polygon-bor-rpc.publicnode.com".into() },
        Network { name: "Avalanche".into(), alias: "avax".into(), rpc_url: "https://api.avax.network/ext/bc/C/rpc".into() },
        Network { name: "Sonic".into(), alias: "sonic".into(), rpc_url: "https://rpc.soniclabs.com".into() },
    ]
}

pub fn config_path(override_path: Option<&Path>) -> PathBuf {
    if let Some(p) = override_path {
        return p.to_path_buf();
    }
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("evm-interactions")
        .join("config.toml")
}

pub fn load_or_default(path: &Path) -> AppConfig {
    if path.exists() {
        AppConfig::load(path).unwrap_or_else(|e| {
            eprintln!("Warning: {e}, using defaults");
            AppConfig::default()
        })
    } else {
        AppConfig::default()
    }
}
