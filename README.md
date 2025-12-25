# EarningsFeed CLI

Command-line interface for the [EarningsFeed API](https://earningsfeed.com) - SEC filings, insider transactions, and institutional holdings.

[Get API Key](https://earningsfeed.com/api) · [API Documentation](https://earningsfeed.com/api/docs) · [Rust SDK](https://crates.io/crates/earningsfeed)

## Installation

### Homebrew (macOS/Linux)

```bash
brew install earningsfeed/tap/earningsfeed
```

### From Source

```bash
cargo install earningsfeed-cli
```

## Quick Start

```bash
# Authenticate with your API key
earningsfeed auth login

# List recent filings
earningsfeed filings list --ticker AAPL

# Get a specific filing
earningsfeed filings get 0000320193-24-000001

# Search for companies
earningsfeed companies search --query "Apple"
```

## Commands

### Authentication

```bash
earningsfeed auth login    # Save your API key
earningsfeed auth logout   # Remove your API key
earningsfeed auth status   # Check authentication status
```

### SEC Filings

```bash
earningsfeed filings list [OPTIONS]
  --ticker <TICKER>    Filter by ticker symbol
  --cik <CIK>          Filter by CIK number
  --forms <FORMS>      Filter by form types (comma-separated)
  --status <STATUS>    Filter by status (all, provisional, final)
  --limit <LIMIT>      Maximum results (1-100, default: 25)
  --cursor <CURSOR>    Pagination cursor

earningsfeed filings get <ACCESSION>
```

### Insider Transactions

```bash
earningsfeed insider list [OPTIONS]
  --ticker <TICKER>      Filter by ticker symbol
  --cik <CIK>            Filter by CIK number
  --person-cik <CIK>     Filter by person CIK
  --direction <DIR>      Filter by direction (buy, sell)
  --min-value <VALUE>    Minimum transaction value (USD)
  --limit <LIMIT>        Maximum results (1-100, default: 25)
  --cursor <CURSOR>      Pagination cursor
```

### Institutional Holdings

```bash
earningsfeed institutional list [OPTIONS]
  --ticker <TICKER>      Filter by ticker symbol
  --cik <CIK>            Filter by CIK number
  --manager-cik <CIK>    Filter by manager CIK
  --put-call <TYPE>      Filter by type (put, call, equity)
  --limit <LIMIT>        Maximum results (1-100, default: 25)
  --cursor <CURSOR>      Pagination cursor
```

### Companies

```bash
earningsfeed companies get <CIK>

earningsfeed companies search [OPTIONS]
  --query <QUERY>    Search query (required)
  --state <STATE>    Filter by state
  --sic <SIC>        Filter by SIC code
  --limit <LIMIT>    Maximum results (1-100, default: 25)
```

## Output Format

All commands output JSON, making it easy to pipe to tools like `jq`:

```bash
# Get recent Apple 10-K filings
earningsfeed filings list --ticker AAPL --forms 10-K | jq '.items[].title'

# Get insider buys over $1M
earningsfeed insider list --direction buy --min-value 1000000 | jq '.items[] | {person: .personName, value: .value}'
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments or config |
| 3 | Authentication error |
| 4 | Rate limit exceeded |
| 5 | Resource not found |
| 6 | Network/timeout error |

## License

MIT
