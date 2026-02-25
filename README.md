# Zerodha CLI

A terminal-based trading tool for Zerodha's Kite Connect API.

## Features

- **Authentication**: OAuth-based login with automatic token refresh
- **Instruments**: Browse and search trading instruments across exchanges
- **Quotes**: Real-time market data (OHLC, LTP, depth)
- **Orders**: Place, modify, and cancel orders
- **Portfolio**: View holdings and positions
- **Margins**: Check available margins
- **GTT**: Good Till Triggered orders
- **Interactive Shell**: REPL mode for power users

## Installation

### From Source

```bash
git clone https://github.com/yourusername/zerodha-cli.git
cd zerodha-cli
cargo install --path cli
```

### Download Binary

See [Releases](https://github.com/yourusername/zerodha-cli/releases) for pre-built binaries.

## Quick Start

```bash
# Configure API credentials
kite auth setup --api-key YOUR_API_KEY --api-secret YOUR_API_SECRET

# Authenticate
kite auth login

# Browse instruments
kite instruments search "INFY"

# Get quote
kite quotes get NSE:INFY

# Place order
kite orders place --symbol NSE:INFY --type BUY --quantity 10 --price 1500

# Interactive shell
kite shell
```

## Documentation

- [Requirements](docs/Requirements.md)
- [Architecture](docs/Architecture.md)
- [Technical Design](docs/Technical-Design.md)
- [Setup Guide](docs/setup.md)

## Project Structure

```
zerodha-cli/
├── Cargo.toml          # Workspace manifest
├── cli/                # CLI binary crate
│   └── src/main.rs
├── core/               # Core library crate
│   └── src/
│       ├── api/        # Kite Connect API client
│       ├── auth/       # Authentication
│       ├── cache/      # Instrument caching
│       ├── config/     # Configuration
│       ├── error/      # Error types
│       ├── models/    # Domain models
│       ├── output/    # Output formatting
│       ├── shell/     # Interactive shell
│       └── validation/# Input validation
└── docs/               # Documentation
```

## Tech Stack

- **Language**: Rust
- **CLI Framework**: clap
- **HTTP Client**: reqwest + tokio
- **Serialization**: serde
- **Output**: comfy-table
- **Interactive Shell**: rustyline

## Requirements

- Rust 1.80+
- Kite Connect API credentials

## License

MIT

## Contributing

Contributions welcome! Please open an issue or PR.
