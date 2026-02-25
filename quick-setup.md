# Quick Setup Guide — Zerodha CLI

Get up and running in 5 minutes!

---

## Step 1: Get Zerodha API Credentials

1. Go to https://developers.kite.trade/
2. Log in with your Zerodha credentials
3. Click **"Create New App"**
4. Fill in details:
   - **App Name:** `zerodha-cli`
   - **Redirect URL:** `http://127.0.0.1`
5. Click **Create**
6. Copy your:
   - `API Key`
   - `API Secret`

---

## Step 2: Configure CLI

### Option A: Environment Variables (Recommended)

```bash
# Add to your shell profile (.zshrc, .bashrc) for permanent setup
export ZERODHA_API_KEY="your_api_key"
export ZERODHA_API_SECRET="your_api_secret"

# Or set for current session only
ZERODHA_API_KEY="your_api_key" ZERODHA_API_SECRET="your_api_secret" cargo run -- auth status
```

**Priority:** Environment variables override config file values.

### Option B: .env File (Auto-loaded)

Create a `.env` file in the project root:

```bash
# ~/Projects/zerodha-cli/.env
ZERODHA_API_KEY=your_api_key
ZERODHA_API_SECRET=your_api_secret
```

The CLI automatically loads `.env` file on startup.

### Option B: Config File

Create `~/.config/zerodha-cli/config.toml`:

```toml
[credentials]
api_key = "your_api_key"
api_secret = "your_api_secret"

[settings]
# Optional: default exchange
default_exchange = "NSE"

# Optional: output format (table/json)
output = "table"
```

---

## Step 3: Run the CLI

```bash
cd ~/Projects/zerodha-cli
cargo run -- auth status
```

---

## Step 4: Authenticate (First Time)

```bash
cargo run -- auth login
```

This will:
1. Open your browser to Zerodha login
2. After login, you'll see a URL like:
   ```
   https://kite.zerodha.com/connect/login?v=3&api_key=XXX&request_token=ABC123
   ```
3. Copy the `request_token` value (ABC123)
4. Paste it into the CLI

That's it! You're authenticated!

---

## Quick Commands Cheat Sheet

```bash
# Check authentication
cargo run -- auth status

# List instruments
cargo run -- instruments list --exchange NSE

# Search for a stock
cargo run -- instruments search INFY

# Get quote
cargo run -- quotes get NSE:INFY

# List your orders
cargo run -- orders list

# View holdings
cargo run -- portfolio holdings

# View positions
cargo run -- portfolio positions

# Check margins
cargo run -- margins list

# Interactive shell
cargo run -- shell
```

---

## Troubleshooting

**"Not authenticated"**
- Run `cargo run -- auth login` first

**"API key not found"**
- Set `ZERODHA_API_KEY` environment variable or add to config file

**"Request token expired"**
- Request tokens expire in a few minutes. Run `cargo run -- auth login` again

---

## Without Cargo (Pre-built Binary)

If you built the binary:
```bash
./target/release/kite auth login
```

Or install globally:
```bash
cargo install --path cli
kite auth login
```
