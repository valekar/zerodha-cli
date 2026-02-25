# Requirements — Zerodha CLI

**Project:** zerodha-cli  
**Version:** 1.0.0  
**Date:** 2026-02-25  
**Author:** ATHENA (Researcher)

---

## 1. Overview

### 1.1 Project Summary

Build a Rust-based CLI tool for Zerodha trading that enables terminal-based interaction with the Kite Connect API. The tool will provide commands for browsing instruments, viewing quotes, managing orders, tracking portfolio, and handling authentication — similar in architecture to polymarket-cli.

### 1.2 Tech Stack

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| **Language** | Rust | Performance, memory safety, single binary distribution |
| **CLI Framework** | clap | Modern derive-based parsing, subcommand support |
| **HTTP Client** | reqwest | Async, TLS support, JSON handling |
| **Serialization** | serde | De/serialization to/from API types |
| **Runtime** | tokio | Async I/O for HTTP operations |
| **Output** | comfy-table | Pretty table formatting |
| **Config** | dirs + toml | Cross-platform config management |
| **Shell** | rustyline | Interactive REPL with history |

### 1.3 Target Users

- Traders who prefer terminal-based workflows
- Algorithmic traders needing CLI scripting
- DevOps/SRE teams monitoring portfolios
- Developers integrating Zerodha into automated workflows

---

## 2. Functional Requirements

### 2.1 Authentication

| ID | User Story | Acceptance Criteria |
|----|------------|---------------------|
| **AUTH-001** | As a user, I want to authenticate with Zerodha so I can access trading APIs | - `kite auth login` opens browser for OAuth flow<br>- User completes login with credentials + TOTP<br>- Access token is stored in config file<br>- `kite auth status` shows authentication state |
| **AUTH-002** | As a user, I want to check if I'm authenticated | - `kite auth status` shows current auth state<br>- Returns "Not authenticated" if no token<br>- Shows token expiry if authenticated |
| **AUTH-003** | As a user, I want to logout and invalidate my session | - `kite auth logout` deletes stored token<br>- Confirms logout with success message |
| **AUTH-004** | As a user, I want to configure API credentials | - `kite auth setup --api-key XXX --api-secret XXX` saves credentials<br>- Credentials stored in `~/.config/zerodha-cli/config.toml` |
| **AUTH-005** | As a user, I want auto-refresh of access tokens | - Token expiration checked before each API call<br>- User warned 24h before expiry<br>- Refresh flow triggers new OAuth if expired |

### 2.2 Instruments

| ID | User Story | Acceptance Criteria |
|----|------------|---------------------|
| **INST-001** | As a trader, I want to list all instruments from an exchange | - `kite instruments list --exchange NSE` shows instruments table<br>- Supports NSE, BSE, NFO, BFO, MCX, CDS<br>- Output formats: table (default), json |
| **INST-002** | As a trader, I want to search for an instrument by symbol or name | - `kite instruments search "INFY"` matches symbols<br>- Search is case-insensitive<br>- Returns matching instruments with exchange info |
| **INST-003** | As a trader, I want to get detailed info for a specific instrument | - `kite instruments get NSE:INFY` shows full details<br>- Includes: name, exchange, segment, instrument_type, lot_size, tick_size, expiry, strike, underlying |
| **INST-004** | As a trader, I want instruments cached locally to avoid repeated downloads | - First `list` downloads and caches CSV<br>- Subsequent `list` uses cache unless `--refresh` flag<br>- Cache stored in `~/.cache/zerodha-cli/instruments/` |

### 2.3 Quotes

| ID | User Story | Acceptance Criteria |
|----|------------|---------------------|
| **QUOTE-001** | As a trader, I want to get full quote for one or more instruments | - `kite quotes get NSE:INFY` shows OHLC, volume, depth, oi<br>- Multiple symbols supported: `NSE:INFY NSE:TCS`<br>- Table format with columns: Symbol, LTP, Change, Change%, Volume, OI |
| **QUOTE-002** | As a trader, I want OHLC data only | - `kite quotes ohlc NSE:INFY` shows OHLC only<br>- Faster, less data transfer |
| **QUOTE-003** | As a trader, I want last traded price only | - `kite quotes ltp NSE:INFY` returns LTP only<br>- Useful for scripts and monitoring |
| **QUOTE-004** | As a trader, I want JSON output for scripting | - `--output json` flag returns JSON<br>- All quote commands support JSON output<br>- Schema matches API response |

### 2.4 Orders

| ID | User Story | Acceptance Criteria |
|----|------------|---------------------|
| **ORDR-001** | As a trader, I want to list all orders for today | - `kite orders list` shows all orders<br>- Filter by status: `--status open|complete|cancelled|rejected`<br>- Table format with: order_id, symbol, type, quantity, price, status, placed_at |
| **ORDR-002** | As a trader, I want to get details for a specific order | - `kite orders get <order_id>` shows full order details<br>- Includes: order_id, status, price, quantity, variety, order_type, product, validity |
| **ORDR-003** | As a trader, I want to place a limit order | - `kite orders place --symbol NSE:INFY --type BUY --order-type LIMIT --quantity 10 --price 1400 --product CNC --validity DAY`<br>- Confirmation prompt before placement<br>- Dry-run flag `--dry-run` for testing |
| **ORDR-004** | As a trader, I want to place a market order | - `kite orders market --symbol NSE:INFY --type BUY --quantity 10 --product MIS`<br>- No price needed (market execution)<br>- Confirmation prompt |
| **ORDR-005** | As a trader, I want to modify an existing order | - `kite orders modify <order_id> --price 1410 --quantity 15`<br>- Only modifiable fields accepted<br>- Shows error if order is closed |
| **ORDR-006** | As a trader, I want to cancel an order | - `kite orders cancel <order_id>` cancels specific order<br>- `kite orders cancel-all` cancels all open orders<br>- Confirmation prompts |
| **ORDR-007** | As a trader, I want to view my trade history | - `kite orders trades` shows all trades<br>- `kite orders trades <order_id>` shows trades for specific order |
| **ORDR-008** | As a trader, I want to specify order variety | - Support: regular, amo (after market order), co (cover order), iceberg<br>- `--variety` flag defaults to regular |

### 2.5 Portfolio

| ID | User Story | Acceptance Criteria |
|----|------------|---------------------|
| **PORT-001** | As a trader, I want to view my holdings (long-term equity) | - `kite portfolio holdings` shows all holdings<br>- Table: Symbol, Qty, Avg Price, LTP, P&L, Day Chg%<br>- P&L highlighted green (profit) / red (loss) |
| **PORT-002** | As a trader, I want to view my positions (intraday/F&O) | - `kite portfolio positions` shows all positions<br>- `--net` shows net positions (default)<br>- `--day` shows day positions only |
| **PORT-003** | As a trader, I want to convert position type | - `kite portfolio convert --symbol NSE:INFY --type BUY --quantity 10 --from NRML --to MIS`<br>- Supports: CNC ↔ MIS ↔ NRML<br>- Confirmation prompt |

### 2.6 Margins

| ID | User Story | Acceptance Criteria |
|----|------------|---------------------|
| **MARG-001** | As a user, I want to view my available margins | - `kite margins list` shows all margin segments<br>- Table: Segment, Total, Used, Available |
| **MARG-002** | As a user, I want to view equity margins specifically | - `kite margins equity` shows equity segment margins only<br>- Includes: cash, collateral, margin_used, net, available |
| **MARG-003** | As a user, I want to view commodity margins | - `kite margins commodity` shows commodity segment margins only |

### 2.7 GTT (Good Till Triggered)

| ID | User Story | Acceptance Criteria |
|----|------------|---------------------|
| **GTT-001** | As a trader, I want to list all GTT orders | - `kite gtt list` shows all GTT orders<br>- Table: Trigger ID, Symbol, Type, Trigger Price, Status, Created |
| **GTT-002** | As a trader, I want to get details for a specific GTT | - `kite gtt get <trigger_id>` shows full GTT details<br>- Includes: conditions, orders, created_at, status |
| **GTT-003** | As a trader, I want to create a single-leg GTT | - `kite gtt create --symbol NSE:INFY --type BUY --quantity 10 --price 1350 --trigger-price 1400 --trigger-type single`<br>- Order placed when trigger price hits |
| **GTT-004** | As a trader, I want to create a two-leg GTT (OCO) | - `kite gtt create --symbol NSE:INFY --type OCO --quantity 10 --price 1350 1450 --trigger-price 1400 1400 --trigger-type two_leg`<br>- Order cancelled when other leg triggers |
| **GTT-005** | As a trader, I want to modify an existing GTT | - `kite gtt modify <trigger_id> --price 1360`<br>- Supports modifying trigger price and order price |
| **GTT-006** | As a trader, I want to delete a GTT order | - `kite gtt delete <trigger_id>` deletes specific GTT<br>- Confirmation prompt |

### 2.8 Interactive Shell

| ID | User Story | Acceptance Criteria |
|----|------------|---------------------|
| **SHELL-001** | As a trader, I want an interactive shell mode | - `kite shell` starts REPL with prompt `kite>`<br>- Supports command history (up/down arrows)<br>- Tab completion for commands and flags |
| **SHELL-002** | As a trader, I want to run any command in shell | - All CLI commands work in shell without `kite` prefix<br>- Examples: `quotes get NSE:INFY`, `orders list` |
| **SHELL-003** | As a trader, I want to exit shell | - `exit`, `quit`, or Ctrl+D exits shell<br>- Session stats shown on exit |
| **SHELL-004** | As a trader, I want shell-specific output | - Persistent output format preference<br>- Last command available for re-execution |

### 2.9 Mutual Funds (Optional/Phase 5)

| ID | User Story | Acceptance Criteria |
|----|------------|---------------------|
| **MF-001** | As a user, I want to list mutual fund orders | - `kite mutual-funds orders` shows MF orders |
| **MF-002** | As a user, I want to list mutual fund holdings | - `kite mutual-funds holdings` shows MF holdings |

---

## 3. Non-Functional Requirements

### 3.1 Performance

| ID | Requirement | Target |
|----|------------|--------|
| **NFR-001** | API response time < 500ms for read operations | 95th percentile < 500ms |
| **NFR-002** | Startup time < 100ms | CLI starts in < 100ms |
| **NFR-003** | Binary size < 10MB | Stripped release binary < 10MB |

### 3.2 Reliability

| ID | Requirement | Target |
|----|------------|--------|
| **NFR-004** | Network error handling | Retry failed requests up to 3 times with exponential backoff |
| **NFR-005** | Graceful degradation | Show cached data if API unavailable (with timestamp) |
| **NFR-006** | Error messages | Human-readable error messages with suggested actions |

### 3.3 Security

| ID | Requirement | Target |
|----|------------|--------|
| **NFR-007** | Credential storage | API secrets stored in OS config directory with appropriate permissions (0600) |
| **NFR-008** | TLS | All HTTP connections use TLS 1.2+ |
| **NFR-009** | Token security | Access tokens never logged or exposed in error messages |
| **NFR-010** | Dry-run mode | All destructive commands (place, cancel, convert) require confirmation or dry-run flag |

### 3.4 Usability

| ID | Requirement | Target |
|----|------------|--------|
| **NFR-011** | Help documentation | All commands have `--help` with examples |
| **NFR-012** | Shell completions | Bash, zsh, fish completions generated and installed |
| **NFR-013** | Table formatting | Auto-fit to terminal width, wrap long content |
| **NFR-014** | JSON output | Consistent JSON schema for scripting |

### 3.5 Maintainability

| ID | Requirement | Target |
|----|------------|--------|
| **NFR-015** | Code coverage | > 70% for core modules (auth, client, types) |
| **NFR-016** | Clippy | Pass clippy with no warnings |
| **NFR-017** | Documentation | All public APIs documented with rustdoc |
| **NFR-018** | Integration tests | End-to-end tests for all command groups |

---

## 4. Data Requirements

### 4.1 Configuration File

**Location:** `~/.config/zerodha-cli/config.toml`

```toml
[api]
api_key = "your_api_key"
api_secret = "your_api_secret"
access_token = "current_access_token"
token_expiry = "2026-02-26T12:00:00Z"

[defaults]
exchange = "NSE"
product = "CNC"
order_type = "LIMIT"
validity = "DAY"

[output]
format = "table"  # or "json"
```

### 4.2 Instrument Cache

**Location:** `~/.cache/zerodha-cli/instruments/<exchange>_<timestamp>.csv`

- CSV format as provided by Kite API
- Cache refreshed with `--refresh` flag or if > 24h old

### 4.3 Shell History

**Location:** `~/.local/share/zerodha-cli/history`

- Max 1000 lines
- Persistent across shell sessions

---

## 5. Constraints & Assumptions

### 5.1 Constraints

| Constraint | Impact |
|------------|--------|
| **Kite Connect API rate limits** | 3 requests per second (hard limit), 2000 requests per day (free tier) |
| **TOTP requirement** | Users must have TOTP setup in their Zerodha account |
| **Market hours** | Order placement only during exchange trading hours |
| **API version** | Must use Kite Connect v3 API |

### 5.2 Assumptions

| Assumption | Validity |
|------------|----------|
| Users have a valid Zerodha account with API access | Required for operation |
| Users are comfortable with terminal-based workflows | Target user characteristic |
| TOTP authenticator app is available on user's device | Required for OAuth flow |
| System has internet connectivity | Required for API access |

---

## 6. Out of Scope

### 6.1 Phase 1 (MVP) Exclusions

- WebSocket streaming for live quotes (deferred to Phase 5)
- Historical data beyond OHLC (deferred)
- Charting/visualization tools (use existing tools)
- Backtesting frameworks (deferred)
- Multi-account support (single account per config)
- GUI or web interface
- Automated trading strategies (CLI only)

### 6.2 Future Considerations

- WebSocket integration for real-time quotes
- Historical data API integration
- Paper trading mode
- Automated order placement strategies
- Multi-asset class alerts
- Portfolio analytics dashboard
- Integration with other brokers

---

## 7. User Interface

### 7.1 Command Structure

```
kite <command> [subcommand] [flags]

Commands:
  auth          Authentication management
  instruments   Browse and search instruments
  quotes        Market data and quotes
  orders        Order management
  portfolio     Holdings and positions
  margins       Margin and funds information
  gtt           Good Till Triggered orders
  mutual-funds  Mutual funds (optional)
  status        Show system status
  shell         Interactive REPL mode
  help          Show help message

Global Flags:
  -o, --output <format>    Output format: table (default), json
  -c, --config <path>      Config file path
  -v, --verbose            Verbose output
  -h, --help               Help for kite
  -V, --version            Version
```

### 7.2 Example Workflows

**Trading Workflow:**
```bash
# 1. Authenticate
kite auth login

# 2. Search for an instrument
kite instruments search "INFY"

# 3. Check current price
kite quotes get NSE:INFY

# 4. Place order
kite orders place \
  --symbol NSE:INFY \
  --type BUY \
  --order-type LIMIT \
  --quantity 10 \
  --price 1400 \
  --product CNC \
  --validity DAY

# 5. Check order status
kite orders list --status open

# 6. View portfolio
kite portfolio holdings
```

**Portfolio Monitoring Workflow:**
```bash
# Check holdings P&L
kite portfolio holdings -o json | jq '.[] | select(.pnl > 0)'

# Check intraday positions
kite portfolio positions --day

# Check margins
kite margins list
```

**Interactive Shell Workflow:**
```bash
kite shell
kite> quotes get NSE:INFY
kite> orders list --status open
kite> portfolio holdings
kite> exit
```

---

## 8. API Endpoint Mapping

| CLI Command | API Endpoint | Method | Notes |
|-------------|-------------|--------|-------|
| `auth login` | `/session/login` | GET | Get login URL |
| `auth login` | `/session/token` | POST | Exchange request_token for access_token |
| `auth logout` | `/session/token` | DELETE | Invalidate session |
| `instruments list` | `/instruments` | GET | CSV download |
| `instruments list --exchange NSE` | `/instruments/NSE` | GET | Exchange-specific CSV |
| `quotes get` | `/quote` | GET | Full quote with depth |
| `quotes ohlc` | `/quote/ohlc` | GET | OHLC data only |
| `quotes ltp` | `/quote/ltp` | GET | Last traded price |
| `orders list` | `/orders` | GET | All orders |
| `orders get` | `/orders/:order_id` | GET | Single order |
| `orders place` | `/orders/:variety` | POST | Place order |
| `orders modify` | `/orders/:variety/:order_id` | PUT | Modify order |
| `orders cancel` | `/orders/:variety/:order_id` | DELETE | Cancel order |
| `orders trades` | `/trades` | GET | All trades |
| `portfolio holdings` | `/portfolio/holdings` | GET | Holdings |
| `portfolio positions` | `/portfolio/positions` | GET | Positions |
| `portfolio convert` | `/portfolio/positions` | PUT | Convert position |
| `margins list` | `/user/margins` | GET | All segments |
| `gtt list` | `/gtt/triggers` | GET | All GTT orders |
| `gtt get` | `/gtt/triggers/:trigger_id` | GET | Single GTT |
| `gtt create` | `/gtt/triggers` | POST | Create GTT |
| `gtt modify` | `/gtt/triggers/:trigger_id` | PUT | Modify GTT |
| `gtt delete` | `/gtt/triggers/:trigger_id` | DELETE | Delete GTT |

---

## 9. Success Criteria

### 9.1 Phase 1 (MVP) Success Metrics

| Metric | Target |
|--------|--------|
| All functional requirements (AUTH-001 to GTT-006) implemented | 100% |
| All command groups working end-to-end | 100% |
| Test coverage > 70% | ✓ |
| Clippy warnings | 0 |
| Binary size < 10MB | ✓ |
| Startup time < 100ms | ✓ |
| API response time < 500ms (95th percentile) | ✓ |

### 9.2 User Acceptance Criteria

1. User can authenticate successfully via OAuth flow
2. User can browse and search instruments with acceptable performance
3. User can place, modify, and cancel orders correctly
4. User can view portfolio and margins accurately
5. User receives clear error messages for failures
6. CLI works on macOS, Linux, and Windows
7. Help documentation is complete and useful

---

## 10. Open Questions

| ID | Question | Priority | Owner |
|----|----------|----------|-------|
| **OQ-001** | Should we implement WebSocket streaming in Phase 1? | Low | TBD |
| **OQ-002** | Should we support multiple accounts in one config? | Low | TBD |
| **OQ-003** | Should we include historical data API endpoints? | Low | TBD |
| **OQ-004** | Should we implement automated trading strategies? | Out of Scope | TBD |

---

## 11. Appendix

### 11.1 Glossary

| Term | Definition |
|------|------------|
| **CNC** | Cash & Carry — product type for delivery-based equity trading |
| **MIS** | Margin Intraday Squareoff — product type for intraday trading |
| **NRML** | Normal — product type for F&O positions |
| **MTF** | Margin Trading Facility — product type for margin-based delivery |
| **GTT** | Good Till Triggered — conditional order type |
| **OCO** | One-Cancels-Other — order cancels when counterpart executes |
| **TOTP** | Time-based One-Time Password — two-factor authentication |
| **OAuth2** | Authorization framework for API access |
| **LTP** | Last Traded Price |
| **OHLC** | Open, High, Low, Close |

### 11.2 References

- **Kite Connect API Documentation:** https://kite.trade/docs/connect/v3/
- **Polymarket CLI:** https://github.com/Polymarket/polymarket-cli
- **clap Documentation:** https://docs.rs/clap/
- **Kite Orders API:** https://kite.trade/docs/connect/v3/orders/
- **Kite Portfolio API:** https://kite.trade/docs/connect/v3/portfolio/
- **Kite Market Quotes:** https://kite.trade/docs/connect/v3/market-quotes/

---

**End of Requirements Document**
