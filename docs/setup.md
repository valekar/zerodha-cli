# Setup Guide — Zerodha CLI

**Project:** zerodha-cli  
**Version:** 1.0.0  
**Date:** 2026-02-25

---

## Prerequisites

| Requirement | Version | Notes |
|-------------|---------|-------|
| **Rust** | 1.80+ | Install via rustup |
| **Cargo** | Included with Rust | Package manager |
| **Internet** | Required | For Kite Connect API |

---

## Installation

### Option 1: Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/zerodha-cli.git
cd zerodha-cli

# Build the binary
cargo build --release

# Install to ~/.cargo/bin
cargo install --path cli

# Verify installation
kite --version
```

### Option 2: Download Binary

```bash
# macOS (Apple Silicon)
curl -L https://github.com/yourusername/zerodha-cli/releases/latest/download/zerodha-cli-aarch64-apple-darwin.tar.gz | tar xz
sudo mv zerodha-cli /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/yourusername/zerodha-cli/releases/latest/download/zerodha-cli-x86_64-apple-darwin.tar.gz | tar xz
sudo mv zerodha-cli /usr/local/bin/

# Linux (x86_64)
curl -L https://github.com/yourusername/zerodha-cli/releases/latest/download/zerodha-cli-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv zerodha-cli /usr/local/bin/
```

### Option 3: Cargo Install

```bash
cargo install zerodha-cli
```

---

## Initial Setup

### Step 1: Get API Credentials

1. Log in to [Kite Connect Dashboard](https://kite.trade/connect/)
2. Create a new app
3. Copy your `API Key` and `API Secret`

### Step 2: Configure CLI

```bash
# Setup API credentials
kite auth setup --api-key YOUR_API_KEY --api-secret YOUR_API_SECRET
```

This creates config at `~/.config/zerodha-cli/config.toml`:

```toml
[api]
api_key = "YOUR_API_KEY"
api_secret = "YOUR_API_SECRET"

[defaults]
exchange = "NSE"
product = "CNC"
order_type = "LIMIT"
validity = "DAY"

[output]
format = "table"
```

### Step 3: Authenticate

```bash
# Start OAuth flow
kite auth login
```

This will:
1. Open your browser to Zerodha login page
2. You enter credentials + TOTP
3. You'll be redirected to a page with a `request_token`
4. Copy and paste the request token back to CLI
5. CLI exchanges it for access token and saves it

---

## Quick Start

### Authentication

```bash
# Check auth status
kite auth status

# Logout
kite auth logout
```

### Browse Instruments

```bash
# List NSE instruments (first run downloads ~2MB CSV)
kite instruments list --exchange NSE

# Search for a symbol
kite instruments search "INFY"

# Get instrument details
kite instruments get NSE:INFY
```

### Get Quotes

```bash
# Full quote with depth
kite quotes get NSE:INFY

# OHLC only
kite quotes ohlc NSE:INFY

# Last traded price
kite quotes ltp NSE:INFY
```

### Place Orders

```bash
# Place limit order
kite orders place \
  --symbol NSE:INFY \
  --type BUY \
  --order-type LIMIT \
  --quantity 10 \
  --price 1500 \
  --product CNC \
  --validity DAY

# Place market order
kite orders market \
  --symbol NSE:INFY \
  --type BUY \
  --quantity 10 \
  --product MIS
```

### View Portfolio

```bash
# Holdings
kite portfolio holdings

# Positions
kite portfolio positions --net
kite portfolio positions --day
```

### Check Margins

```bash
# All margins
kite margins list

# Equity only
kite margins equity

# Commodity only
kite margins commodity
```

### Interactive Shell

```bash
# Start REPL
kite shell

# In shell:
kite> quotes get NSE:INFY
kite> orders list --status open
kite> portfolio holdings
kite> exit
```

---

## Output Formats

All commands support `--output` / `-o` flag:

```bash
# Table format (default)
kite quotes get NSE:INFY

# JSON format (for scripting)
kite quotes get NSE:INFY -o json | jq '.last_price'

# Use with any command
kite portfolio holdings -o json
kite orders list -o json
```

---

## Configuration

### Config File Location

| OS | Path |
|----|------|
| **Linux** | `~/.config/zerodha-cli/config.toml` |
| **macOS** | `~/.config/zerodha-cli/config.toml` |
| **Windows** | `%APPDATA%\zerodha-cli\config.toml` |

### Manual Config Editing

```toml
[api]
api_key = "your_api_key"
api_secret = "your_api_secret"
# access_token and token_expiry are added automatically

[defaults]
exchange = "NSE"
product = "CNC"
order_type = "LIMIT"
validity = "DAY"

[output]
format = "table"  # or "json"
```

### Environment Variables

Override config with environment variables:

```bash
export ZERODHA_API_KEY=your_key
export ZERODHA_API_SECRET=your_secret

kite auth status
```

---

## Cache Management

### Instrument Cache

Instruments are cached in `~/.cache/zerodha-cli/instruments/`:

```bash
# Force refresh (re-download CSV)
kite instruments list --exchange NSE --refresh
```

Cache expires after 24 hours.

### Clear Cache

```bash
rm -rf ~/.cache/zerodha-cli/
```

---

## Shell History

Interactive shell history is saved to `~/.local/share/zerodha-cli/history`.

Max 1000 lines, persistent across sessions.

---

## Troubleshooting

### Authentication Issues

```bash
# Re-authenticate
kite auth logout
kite auth login
```

### API Errors

```bash
# Run with verbose output
kite -v quotes get NSE:INFY
```

Common errors:
- **"Invalid access token"**: Run `kite auth login`
- **"Symbol not found"**: Check symbol format (e.g., `NSE:INFY`)
- **"Rate limit exceeded"**: Wait 1 second between requests

### Network Issues

```bash
# Check internet connectivity
curl -I https://kite.zerodha.com
```

---

## Docker (Alternative)

While zerodha-cli is a native binary, you can run it in Docker:

```bash
# Build
docker build -t zerodha-cli .

# Run
docker run --rm -it \
  -v ~/.config/zerodha-cli:/root/.config/zerodha-cli \
  zerodha-cli auth status
```

---

## Development

### Project Structure

```
zerodha-cli/
├── Cargo.toml          # Workspace manifest
├── cli/                # CLI binary crate
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── core/               # Core library crate
│   ├── Cargo.toml
│   └── src/
│       ├── api/
│       ├── auth/
│       ├── cache/
│       ├── cli/
│       ├── config/
│       ├── error/
│       ├── models/
│       ├── output/
│       ├── shell/
│       └── validation/
├── tests/              # Integration tests
└── docs/               # Documentation
```

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- auth status
```

### Clippy & Formatting

```bash
# Format code
cargo fmt

# Lint
cargo clippy --all-targets

# Check
cargo check
```

---

## Commands Reference

| Command | Description |
|---------|-------------|
| `kite auth login` | Authenticate with OAuth |
| `kite auth status` | Check auth status |
| `kite auth logout` | Invalidate session |
| `kite instruments list` | List instruments |
| `kite instruments search` | Search instruments |
| `kite instruments get` | Get instrument details |
| `kite quotes get` | Get full quote |
| `kite quotes ohlc` | Get OHLC data |
| `kite quotes ltp` | Get LTP |
| `kite orders list` | List orders |
| `kite orders get` | Get order details |
| `kite orders place` | Place limit order |
| `kite orders market` | Place market order |
| `kite orders modify` | Modify order |
| `kite orders cancel` | Cancel order |
| `kite portfolio holdings` | View holdings |
| `kite portfolio positions` | View positions |
| `kite margins list` | View all margins |
| `kite margins equity` | View equity margins |
| `kite margins commodity` | View commodity margins |
| `kite gtt list` | List GTT orders |
| `kite gtt create` | Create GTT |
| `kite gtt modify` | Modify GTT |
| `kite gtt delete` | Delete GTT |
| `kite shell` | Interactive REPL |
| `kite status` | System status |

---

## Next Steps

After setup:

1. ✅ Authenticate with `kite auth login`
2. ✅ Explore instruments with `kite instruments search`
3. ✅ Check quotes with `kite quotes get NSE:INFY`
4. ✅ Place a test order (use `--dry-run` flag first)
5. ✅ Try interactive shell with `kite shell`

---

**End of Setup Guide**
