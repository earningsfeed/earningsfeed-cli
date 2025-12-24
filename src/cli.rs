//! Command-line argument definitions using clap derive.

use clap::{Parser, Subcommand, ValueEnum};

/// CLI for the EarningsFeed API - SEC filings, insider transactions, and institutional holdings.
#[derive(Parser)]
#[command(name = "earningsfeed")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Manage authentication
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
    /// SEC filings
    Filings {
        #[command(subcommand)]
        action: FilingsAction,
    },
    /// Insider transactions
    Insider {
        #[command(subcommand)]
        action: InsiderAction,
    },
    /// Institutional holdings
    Institutional {
        #[command(subcommand)]
        action: InstitutionalAction,
    },
    /// Company lookup
    Companies {
        #[command(subcommand)]
        action: CompaniesAction,
    },
}

// ============ Auth ============

#[derive(Subcommand)]
pub enum AuthAction {
    /// Log in with your API key
    Login,
    /// Log out and remove stored API key
    Logout,
    /// Show authentication status
    Status,
}

// ============ Filings ============

#[derive(Subcommand)]
pub enum FilingsAction {
    /// List SEC filings
    List {
        /// Filter by ticker symbol
        #[arg(long)]
        ticker: Option<String>,

        /// Filter by CIK number
        #[arg(long)]
        cik: Option<u64>,

        /// Filter by form types (comma-separated, e.g., "10-K,10-Q")
        #[arg(long, value_delimiter = ',')]
        forms: Option<Vec<String>>,

        /// Filter by filing status
        #[arg(long)]
        status: Option<FilingStatusArg>,

        /// Maximum number of results (1-100)
        #[arg(long, default_value = "25")]
        limit: u32,

        /// Cursor for pagination
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Get a specific filing by accession number
    Get {
        /// Accession number (e.g., 0000320193-24-000001)
        accession: String,
    },
}

#[derive(Clone, ValueEnum)]
pub enum FilingStatusArg {
    All,
    Provisional,
    Final,
}

// ============ Insider ============

#[derive(Subcommand)]
pub enum InsiderAction {
    /// List insider transactions
    List {
        /// Filter by ticker symbol
        #[arg(long)]
        ticker: Option<String>,

        /// Filter by CIK number
        #[arg(long)]
        cik: Option<u64>,

        /// Filter by person CIK
        #[arg(long)]
        person_cik: Option<u64>,

        /// Filter by transaction direction (buy/sell)
        #[arg(long)]
        direction: Option<TransactionDirectionArg>,

        /// Minimum transaction value in USD
        #[arg(long)]
        min_value: Option<u64>,

        /// Maximum number of results (1-100)
        #[arg(long, default_value = "25")]
        limit: u32,

        /// Cursor for pagination
        #[arg(long)]
        cursor: Option<String>,
    },
}

#[derive(Clone, ValueEnum)]
pub enum TransactionDirectionArg {
    Buy,
    Sell,
}

// ============ Institutional ============

#[derive(Subcommand)]
pub enum InstitutionalAction {
    /// List institutional holdings
    List {
        /// Filter by ticker symbol
        #[arg(long)]
        ticker: Option<String>,

        /// Filter by CIK number
        #[arg(long)]
        cik: Option<u64>,

        /// Filter by manager CIK
        #[arg(long)]
        manager_cik: Option<u64>,

        /// Filter by put/call type
        #[arg(long)]
        put_call: Option<PutCallArg>,

        /// Maximum number of results (1-100)
        #[arg(long, default_value = "25")]
        limit: u32,

        /// Cursor for pagination
        #[arg(long)]
        cursor: Option<String>,
    },
}

#[derive(Clone, ValueEnum)]
pub enum PutCallArg {
    Put,
    Call,
    Equity,
}

// ============ Companies ============

#[derive(Subcommand)]
pub enum CompaniesAction {
    /// Get a company by CIK
    Get {
        /// Company CIK number
        cik: u64,
    },
    /// Search for companies
    Search {
        /// Search query
        #[arg(long)]
        query: String,

        /// Filter by state
        #[arg(long)]
        state: Option<String>,

        /// Filter by SIC code
        #[arg(long)]
        sic: Option<u32>,

        /// Maximum number of results (1-100)
        #[arg(long, default_value = "25")]
        limit: u32,
    },
}
