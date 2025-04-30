# Pumpfun Monitoring Tool

This is a Pumpfun contract monitoring tool based on the Carbon framework, used for listening to and analyzing events and transactions on the Pumpfun contract. The tool can monitor key events such as token creation, trading, bonding curve completion, and more, while providing data processing and analysis functionality.

## Features

- Monitor all events and transactions on the Pumpfun contract
- Parse and process key events like Launch, Swap, Complete, Withdraw
- Identify large transactions and provide alerts
- Support for custom sidechain RPC endpoint configuration
- Flexible logging and data processing

## Requirements

- Rust 1.50+
- Cargo
- Helius API Key (for connecting to Solana or a sidechain)

## Installation and Setup

1. Clone the repository

```bash
git clone [Repository URL]
cd pumpfun_monitor
```

2. Set up environment variables

Copy the `.env.example` file and rename it to `.env`:

```bash
cp .env.example .env
```

Edit the `.env` file and set the necessary environment variables:

```
API_KEY=Your_Helius_API_Key
RUST_LOG=info
# If connecting to a sidechain, uncomment the next line and enter the correct RPC address
# CUSTOM_RPC_ENDPOINT=https://your-custom-sidechain-rpc-endpoint.com
```

3. Compile the program

```bash
cargo build --release
```

## Usage

Run the monitoring tool:

```bash
cargo run --release
```

The program will start listening for events and transactions on the Pumpfun contract and process them according to the configuration.

## Customization

### Monitor Specific Tokens

Set the `TARGET_TOKENS` variable in the `.env` file, listing the token addresses to monitor, separated by commas:

```
TARGET_TOKENS=token_mint_address1,token_mint_address2
```

### Adjust Transaction Monitoring Threshold

By default, the program logs transactions larger than 10 SOL. You can adjust this threshold by setting the `SWAP_THRESHOLD` environment variable:

```
SWAP_THRESHOLD=5000000000  # Set to 5 SOL
```

## Code Structure

- `main.rs` - Program entry point and Carbon Pipeline configuration
- `types.rs` - Definitions of Pumpfun contract-related data structures
- `decoder.rs` - Implementation of instruction and event decoding logic

## Extensions and Considerations

1. The event parsing logic in the current version is just a framework and needs to be refined based on the actual log format
2. When connecting to a sidechain, ensure that the RPC endpoint is configured correctly and that the API is compatible with Solana
3. For large-scale data processing, consider adding database storage or other persistence mechanisms

## License

[License Information] 