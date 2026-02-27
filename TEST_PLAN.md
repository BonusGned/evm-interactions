# Test Plan: evm-interactions

This document outlines the test cases required to ensure full coverage for the project. Since the project is a Rust-based CLI utility for interacting with EVM-compatible networks, the testing strategy is divided into unit tests, CLI integration tests, and RPC interaction testing.

---

## 1. Unit Tests

Unit tests cover individual functions and data structures, verifying their logic in isolation.

### 1.1. Configuration (`src/config.rs`)

* **`test_default_config_creation`**: Verify that `AppConfig::default()` creates a configuration with the expected default networks (Ethereum, BSC, Polygon, Avalanche, Sonic) and that `default_network` is set to "eth".
* **`test_config_load_save`**: Create a temporary file (using `tempfile`), call `save()` for a custom `AppConfig`, then `load()` it back and verify the data integrity.
* **`test_find_network_by_name`**: Verify that `find_network` locates a network by its full name.
* **`test_find_network_by_alias`**: Verify that `find_network` locates a network by its alias.
* **`test_find_network_case_insensitive`**: Verify that `find_network` works with various case combinations (uppercase, mixed case).
* **`test_find_network_not_found`**: Verify that `find_network` returns `None` for non-existent networks.
* **`test_add_network_success`**: Successfully add a new network with valid parameters.
* **`test_add_network_duplicate_by_alias`**: Attempt to add a network with an existing alias; the function should return an error (`Err`).
* **`test_add_network_duplicate_by_name`**: Attempt to add a network with an existing name; the function should return an error (`Err`).
* **`test_remove_network_success`**: Remove an existing network by alias; the function should return `true` and the network should be removed from the list.
* **`test_remove_network_by_name`**: Remove an existing network by name; should also work.
* **`test_remove_network_not_found`**: Attempt to remove a non-existent network; should return `false`.
* **`test_resolve_networks_all`**: Check the `resolve_networks` function with the `all = true` flag; it should return all networks in the configuration.
* **`test_resolve_networks_by_aliases`**: Check requesting specific networks via a list of aliases.
* **`test_resolve_networks_with_custom_rpc`**: Check the `resolve_networks` function when a custom `rpc_url` is passed; it should create temporary custom network entries.
* **`test_resolve_networks_default_fallback`**: When no aliases and `all = false`, the function should return only the default network.
* **`test_resolve_networks_no_default_returns_all`**: When `default_network` is `None` and no aliases are provided, it should return all networks.
* **`test_resolve_network_custom_rpc`**: Check `resolve_network` when a custom `rpc_url` is passed; it should return a temporary `Custom` network.
* **`test_resolve_network_custom_rpc_with_alias`**: Custom RPC with a named alias should use that alias as both name and alias.
* **`test_resolve_network_by_alias`**: Resolve a single network by alias.
* **`test_resolve_network_default`**: When no alias or rpc is provided, resolve using the default network.
* **`test_resolve_network_no_default_fallback_first`**: When default is `None`, fall back to the first configured network.
* **`test_config_path_with_override`**: Test `config_path` with an explicit override returns the override path.
* **`test_config_path_default`**: Test `config_path` without override returns a path ending in `evm-interactions/config.toml`.
* **`test_load_or_default_missing_file`**: When config file doesn't exist, `load_or_default` returns the default config.
* **`test_load_or_default_existing_file`**: When config file exists, `load_or_default` returns the saved config.
* **`test_load_nonexistent_file_returns_error`**: `AppConfig::load` on a missing file returns `Err`.
* **`test_save_creates_parent_dirs`**: `save()` should create intermediate directories if they don't exist.

### 1.2. Command Utilities (`src/commands/block.rs`)

* **`test_parse_block_number_hex`**: The `parse_block_number` function should return the value unchanged if the input is `0x123` or `0xff`.
* **`test_parse_block_number_decimal`**: When a decimal number is input (e.g., `123`), the function should return its hex representation (`0x7b`). Also tests `0` → `0x0` and `1000000` → `0xf4240`.
* **`test_parse_block_number_keywords`**: When `latest`, `earliest`, or `pending` are input, return them as strings unchanged.
* **`test_parse_block_number_invalid`**: When an invalid string is input (neither a number nor a keyword), it should return the original string (which will subsequently fail during the RPC call, as intended).

### 1.3. Model Utilities (`src/model.rs`)

* **`test_hex_to_u64`**: Verify hex-to-u64 conversion for valid hex values, zero, and invalid input (returns 0).
* **`test_hex_to_u128`**: Verify hex-to-u128 conversion including large values (1 ETH in wei) and invalid input.
* **`test_wei_hex_to_ether`**: Verify conversion from wei hex string to ether (1 ETH = `0xde0b6b3a7640000`).
* **`test_wei_hex_to_gwei`**: Verify conversion from wei hex string to gwei (1 Gwei = `0x3b9aca00`).
* **`test_block_number_dec`**: `Block::number_dec()` correctly converts hex block number to decimal.
* **`test_block_timestamp_dec`**: `Block::timestamp_dec()` correctly converts hex timestamp.
* **`test_block_gas_used_dec`**: `Block::gas_used_dec()` correctly converts hex gas used.
* **`test_block_gas_limit_dec`**: `Block::gas_limit_dec()` correctly converts hex gas limit.
* **`test_block_base_fee_gwei`**: `Block::base_fee_gwei()` returns correct gwei value when present.
* **`test_block_base_fee_none`**: `Block::base_fee_gwei()` returns `None` when `base_fee_per_gas` is absent.
* **`test_block_gas_usage_percent`**: `Block::gas_usage_percent()` correctly calculates the ratio.
* **`test_block_gas_usage_zero_limit`**: Returns `0.0` when gas limit is zero (avoids division by zero).
* **`test_tx_value_ether`**: `Transaction::value_ether()` correctly converts hex value to ether.
* **`test_tx_gas_limit_dec`**: `Transaction::gas_limit_dec()` correctly converts hex gas limit.
* **`test_tx_gas_price_gwei`**: `Transaction::gas_price_gwei()` returns gwei value when present.
* **`test_tx_gas_price_none`**: Returns `None` when gas price is absent.
* **`test_tx_nonce_dec`**: `Transaction::nonce_dec()` correctly converts hex nonce.
* **`test_tx_block_number_dec`**: Returns `Some(n)` for confirmed transactions.
* **`test_tx_block_number_pending`**: Returns `None` for pending transactions.
* **`test_tx_input_preview_transfer`**: Input `0x` returns `"0x (transfer)"`.
* **`test_tx_input_preview_empty`**: Empty input returns `"0x (transfer)"`.
* **`test_tx_input_preview_method_selector`**: Long input is truncated to first 10 chars (method selector).
* **`test_tx_input_preview_short`**: Short input (≤10 chars) is returned as-is.
* **`test_receipt_succeeded_true`**: Status `0x1` → `Some(true)`.
* **`test_receipt_succeeded_false`**: Status `0x0` → `Some(false)`.
* **`test_receipt_succeeded_none`**: No status → `None`.
* **`test_receipt_gas_used_dec`**: Correctly converts hex gas used.
* **`test_receipt_effective_gas_price_gwei`**: Returns correct gwei value.
* **`test_receipt_tx_cost_ether`**: Correctly computes `gas_price * gas_used` in ether.
* **`test_receipt_tx_cost_no_gas_price`**: Returns `0.0` when gas price is absent.

### 1.4. Display Formatting (`src/display.rs`)

* **`test_format_number_zero`**: `format_number(0)` → `"0"`.
* **`test_format_number_small`**: Numbers under 1000 have no comma separators.
* **`test_format_number_thousands`**: Numbers 1000+ are formatted with comma separators (e.g., `1,234,567`).
* **`test_format_ether_zero`**: `format_ether(0.0)` → `"0"`.
* **`test_format_ether_small_value`**: Very small values (< 0.0001) are formatted with up to 10 decimal places, trailing zeros trimmed.
* **`test_format_ether_normal_value`**: Normal values are formatted with up to 6 decimal places, trailing zeros trimmed.
* **`test_format_ether_precise`**: Full precision values are preserved.

---

## 2. Integration Tests

### 2.1. CLI Command Logic (`src/commands/config.rs`)

To test CLI commands, use isolated temporary directories for configurations (by passing a `cfg_path`) to avoid overwriting the user's actual config.

* **`test_cli_config_init`**: Call `init(temp_path)`. Verify that the config file is created and contains the expected data.
* **`test_cli_config_add`**: Call `add` to include a network, read the updated `temp_path`, and verify the new network exists.
* **`test_cli_config_remove`**: Remove a network via the command and verify the file update.
* **`test_cli_config_default_set`**: Set a default network and verify that the `default_network` field is updated in the file.
* **`test_cli_config_default_show`**: Show the current default network (no panic, happy path).
* **`test_cli_config_list`**: List all networks (no panic, happy path).
* **`test_cli_config_path`**: Show config path (no panic, happy path).

> **Note:** Error-path tests (e.g., `test_cli_config_add_invalid_url`, duplicate add, remove non-existent) cannot be tested directly because the command functions call `std::process::exit(1)`. For full CLI pipeline testing with exit code verification, the `assert_cmd` crate is recommended.

### 2.2. RPC Client (`src/rpc.rs`)

HTTP responses are mocked using the `wiremock` crate to avoid making real network requests.

* **`test_rpc_get_block_success`**: Mock server returns a valid JSON-RPC response for `eth_getBlockByNumber`. Verify the client deserializes it correctly.
* **`test_rpc_get_block_rpc_error`**: Mock server returns a JSON-RPC response with an `error` field. Verify the client handles the blockchain error correctly.
* **`test_rpc_get_block_empty_result`**: Mock server returns `result: null`. Verify the client returns an "empty result" error.
* **`test_rpc_server_error`**: Mock server returns HTTP 500. Verify the client handles it with "failed to parse response".
* **`test_rpc_connection_refused`**: Attempt to connect to a non-listening port. Verify the client returns "request failed".
* **`test_rpc_get_transaction_success`**: Mock a valid `eth_getTransactionByHash` response and verify deserialization.
* **`test_rpc_get_transaction_receipt_success`**: Mock a valid `eth_getTransactionReceipt` response and verify deserialization.
* **`test_rpc_get_balance_success`**: Mock a valid `eth_getBalance` response and verify the returned hex string.
* **`test_rpc_get_gas_price_success`**: Mock a valid `eth_gasPrice` response and verify the returned hex string.
* **`test_rpc_get_max_priority_fee_success`**: Mock a valid `eth_maxPriorityFeePerGas` response and verify the returned hex string.

---

### Workflow Summary:

1. **Triggers**: Executes on every `push` to the `main` branch and for every `pull_request` targeting `main`.
2. **Checkout**: Uses `actions/checkout` to fetch the repository's source code into the runner.
3. **Rust Toolchain Setup**: Installs the latest stable Rust version, including necessary components like `rustfmt` and `clippy`.
4. **Caching**: Implements `rust-cache` to store dependencies and build artifacts, significantly reducing subsequent run times.
5. **Formatting Check**: Runs `cargo fmt --check` to ensure the code adheres to standard Rust formatting style.
6. **Linting (Clippy)**: Executes `cargo clippy` with a "deny warnings" flag, causing the build to fail if any code smells or potential bugs are detected.
7. **Test Execution**: Runs the complete suite of unit and integration tests using `cargo test --verbose` to ensure logic correctness.

---
