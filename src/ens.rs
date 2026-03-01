use sha3::{Digest, Keccak256};

pub const ENS_REGISTRY: &str = "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e";
const RESOLVER_SELECTOR: &str = "0x0178b8bf";
const ADDR_SELECTOR: &str = "0x3b3b57de";

pub fn namehash(name: &str) -> [u8; 32] {
    let mut node = [0u8; 32];
    if name.is_empty() {
        return node;
    }
    for label in name.rsplit('.') {
        let label_hash = Keccak256::digest(label.as_bytes());
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&node);
        combined.extend_from_slice(&label_hash);
        node = Keccak256::digest(&combined).into();
    }
    node
}

pub fn encode_resolver_call(name: &str) -> String {
    let hash = namehash(name);
    format!("{}{}", RESOLVER_SELECTOR, hex::encode(&hash))
}

pub fn encode_addr_call(name: &str) -> String {
    let hash = namehash(name);
    format!("{}{}", ADDR_SELECTOR, hex::encode(&hash))
}

pub fn parse_address_from_result(hex_result: &str) -> Option<String> {
    let clean = hex_result.trim_start_matches("0x");
    if clean.len() < 64 {
        return None;
    }
    let addr = &clean[24..64];
    if addr.chars().all(|c| c == '0') {
        return None;
    }
    Some(format!("0x{addr}"))
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        super::hex_encode(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namehash_empty() {
        let hash = namehash("");
        assert_eq!(hash, [0u8; 32]);
    }

    #[test]
    fn test_namehash_eth() {
        let hash = namehash("eth");
        let hex = hex_encode(&hash);
        assert_eq!(
            hex,
            "93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae"
        );
    }

    #[test]
    fn test_namehash_full_domain() {
        let hash = namehash("vitalik.eth");
        let hex = hex_encode(&hash);
        assert_eq!(
            hex,
            "ee6c4522aab0003e8d14cd40a6af439055fd2577951148c14b6cea9a53475835"
        );
    }

    #[test]
    fn test_encode_resolver_call() {
        let data = encode_resolver_call("vitalik.eth");
        assert!(data.starts_with(RESOLVER_SELECTOR));
        assert_eq!(data.len(), 10 + 64);
    }

    #[test]
    fn test_encode_addr_call() {
        let data = encode_addr_call("vitalik.eth");
        assert!(data.starts_with(ADDR_SELECTOR));
        assert_eq!(data.len(), 10 + 64);
    }

    #[test]
    fn test_parse_address_from_result() {
        let result = "0x000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045";
        let addr = parse_address_from_result(result).unwrap();
        assert_eq!(addr, "0xd8da6bf26964af9d7eed9e03e53415d37aa96045");
    }

    #[test]
    fn test_parse_address_zero() {
        let result = "0x0000000000000000000000000000000000000000000000000000000000000000";
        assert!(parse_address_from_result(result).is_none());
    }

    #[test]
    fn test_parse_address_too_short() {
        assert!(parse_address_from_result("0x1234").is_none());
    }
}
