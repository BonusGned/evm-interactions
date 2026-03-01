# evm-interactions

CLI tool for interacting with EVM-compatible blockchains. Fetch latest block data, manage network configurations, and explore on-chain state — all from your terminal.

> This is an evolving project. The goal is to build a comprehensive CLI toolkit for Web3 interactions: reading blockchain state, decoding transactions, monitoring events, and more.

## Installation

### From source

```bash
git clone https://github.com/user/evm-interactions.git
cd evm-interactions
cargo install --path . --locked
```

### From GitHub Releases

Download the pre-built binary for your platform from the [Releases](https://github.com/user/evm-interactions/releases) page.

## Quick Start

```bash
# Initialize config with default networks (Ethereum, BSC, Polygon, Avalanche, Sonic)
evm-interactions config init

# Fetch latest block from default network
evm-interactions block

# Fetch from all configured networks
evm-interactions block --all

# Fetch from a specific network by alias
evm-interactions block -a eth
```

## Usage

### Fetch Blocks

```bash
# Query default network
evm-interactions block

# Query specific network(s) by alias
evm-interactions block -a eth
evm-interactions block -a eth -a bsc

# Query all configured networks
evm-interactions block --all

# Fetch a specific block by number
evm-interactions block -n 24500000
evm-interactions block -n 0x175D7A0 -a eth

# One-off query with custom RPC
evm-interactions block -a arb --rpc https://arb1.arbitrum.io/rpc
```

### Transaction Lookup

```bash
# Look up a transaction by hash (uses default network)
evm-interactions tx 0xabc123...

# Look up on a specific network
evm-interactions tx 0xabc123... -a eth

# With custom RPC
evm-interactions tx 0xabc123... --rpc https://arb1.arbitrum.io/rpc
```

### Account Balance

```bash
# Query balance on default network
evm-interactions balance 0x1234...

# Query on specific network(s)
evm-interactions balance 0x1234... -a eth
evm-interactions balance 0x1234... -a eth -a bsc

# Query across all configured networks
evm-interactions balance 0x1234... --all
```

### Gas Prices

```bash
# Show gas price for default network
evm-interactions gas

# Show gas prices for specific networks
evm-interactions gas -a eth -a bsc

# Show gas prices for all configured networks
evm-interactions gas --all
```

### Contract Read Calls

```bash
# Call a contract function (raw calldata)
evm-interactions call 0xContractAddress 0x12345678

# Call on a specific network
evm-interactions call 0xContractAddress 0x12345678 -a eth

# Call at a specific block
evm-interactions call 0xContractAddress 0x12345678 -b 24500000

# Output as JSON
evm-interactions -o json call 0xContractAddress 0x12345678
```

### Event Logs

```bash
# Fetch logs from a contract
evm-interactions logs 0xContractAddress --from 24500000 --to latest

# Filter by topic
evm-interactions logs 0xContractAddress --topic 0xddf252ad... --from 24500000 --to 24500100

# Output as CSV
evm-interactions -o csv logs 0xContractAddress --from 24500000 --to latest -a eth
```

### ENS Resolution

```bash
# Resolve an ENS name
evm-interactions ens vitalik.eth

# Resolve on a specific network
evm-interactions ens vitalik.eth -a eth

# Output as JSON
evm-interactions -o json ens vitalik.eth
```

### Watch Mode

```bash
# Watch for new blocks in real time (default: 12s interval)
evm-interactions block --watch

# Watch with custom interval
evm-interactions block --watch --interval 5

# Watch a specific network
evm-interactions block --watch -a eth

# Watch all networks
evm-interactions block --watch --all
```

### Export Data

```bash
# Output any command as JSON
evm-interactions -o json block
evm-interactions -o json balance 0x1234...
evm-interactions -o json gas --all

# Output any command as CSV
evm-interactions -o csv block --all
evm-interactions -o csv balance 0x1234... --all
evm-interactions -o csv gas --all
```

### Manage Config

```bash
# Initialize config with defaults
evm-interactions config init

# List all networks
evm-interactions config list

# Add a new network
evm-interactions config add -n Arbitrum -a arb -r https://arb1.arbitrum.io/rpc

# Remove a network
evm-interactions config remove -a arb

# Get/set default network
evm-interactions config default
evm-interactions config default -a bsc

# Show config file path
evm-interactions config path

# Use a custom config file
evm-interactions -c ./my-config.toml block --all
```

### Config File

Config is stored at `~/.config/evm-interactions/config.toml` (Linux/macOS) by default.

```toml
default_network = "eth"

[[networks]]
name = "Ethereum"
alias = "eth"
rpc_url = "https://ethereum-rpc.publicnode.com"

[[networks]]
name = "BSC"
alias = "bsc"
rpc_url = "https://bsc-dataseed1.binance.org"

[[networks]]
name = "Polygon"
alias = "matic"
rpc_url = "https://polygon-bor-rpc.publicnode.com"

[[networks]]
name = "Avalanche"
alias = "avax"
rpc_url = "https://api.avax.network/ext/bc/C/rpc"

[[networks]]
name = "Sonic"
alias = "sonic"
rpc_url = "https://rpc.soniclabs.com"
```

## Output Examples

### Block

```
══════════════════════════════════════════════════════════
            ⛓  evm-interactions · EVM explorer  ⛓
══════════════════════════════════════════════════════════

  Network: Ethereum
  Block:   #24492330
  Hash:    0x85ebfb41fddb4dc31e9da3e43375141283d68ef2...
  Time:    2026-02-19 17:16:59 UTC
  Miner:   0xe688b84b23f322a994a53dbf8e15fa82cdb71127
  Gas:     14,712,381 / 60,000,000 (24.5%)
  Base Fee: 0.1549 Gwei
  Txns:    265
```

### Transaction

```
══════════════════════════════════════════════════════════
            ⛓  evm-interactions · EVM explorer  ⛓
══════════════════════════════════════════════════════════

  Network:   Ethereum
  Tx Hash:   0xabc123...
  Status:    Success ✓
  Block:     #24492330
  From:      0x1234...
  To:        0x5678...
  Value:     1.5 ETH
  Gas:       21,000 / 21,000 (100%)
  Gas Price: 15.5000 Gwei
  Tx Cost:   0.000326 ETH
  Nonce:     42
  Input:     0x (transfer)
```

### Balance

```
══════════════════════════════════════════════════════════
            ⛓  evm-interactions · EVM explorer  ⛓
══════════════════════════════════════════════════════════

  Network: Ethereum
  Address: 0x1234...
  Balance: 1.234567 ETH
  ──────────────────────────────────────────────────────────
  Network: BSC
  Address: 0x1234...
  Balance: 0.5 ETH
```

### Gas Prices

```
══════════════════════════════════════════════════════════
            ⛓  evm-interactions · EVM explorer  ⛓
══════════════════════════════════════════════════════════

  Network:   Ethereum
  Gas Price: 15.5000 Gwei
  Priority:  1.0000 Gwei
  Base Fee:  14.5000 Gwei
  ──────────────────────────────────────────────────────────
  Network:   BSC
  Gas Price: 1.0000 Gwei
```

### Contract Call

```
══════════════════════════════════════════════════════════
            ⛓  evm-interactions · EVM explorer  ⛓
══════════════════════════════════════════════════════════

  Network:   Ethereum
  Contract:  0xdAC17F958D2ee523a2206206994597C13D831ec7
  Calldata:  0x18160ddd...
  Result:    0x000000000000000000000000000000000000000000000000000000174876e800
```

### ENS Resolution

```
══════════════════════════════════════════════════════════
            ⛓  evm-interactions · EVM explorer  ⛓
══════════════════════════════════════════════════════════

  Name:    vitalik.eth
  Address: 0xd8da6bf26964af9d7eed9e03e53415d37aa96045
```

### JSON Output

```json
{"network":"Ethereum","number":24492330,"hash":"0x85ebfb41...","timestamp":1739983019,"miner":"0xe688b84b...","gasUsed":14712381,"gasLimit":60000000,"baseFeeGwei":0.1549,"txCount":265}
```

## Roadmap

- [x] Fetch latest block from multiple EVM networks
- [x] Configurable network list with TOML config
- [x] Support network aliases (e.g., eth, bsc)
- [x] CLI with subcommands
- [x] Fetch specific block by number
- [x] Transaction lookup
- [x] Account balance queries
- [x] Gas price tracking and estimation
- [x] Contract read calls (eth_call)
- [x] Event log filtering and monitoring
- [x] ENS resolution
- [x] Watch mode (live block stream)
- [x] Export data to JSON/CSV

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## License

MIT
