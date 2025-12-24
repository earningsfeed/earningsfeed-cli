//! Filings command implementation.

use earningsfeed::{EarningsFeed, FilingStatus, ListFilingsParams};

use crate::cli::{FilingStatusArg, FilingsAction};
use crate::config::Config;
use crate::error::Result;

pub async fn run(action: FilingsAction) -> Result<()> {
    match action {
        FilingsAction::List {
            ticker,
            cik,
            forms,
            status,
            limit,
            cursor,
        } => list(ticker, cik, forms, status, limit, cursor).await,
        FilingsAction::Get { accession } => get(accession).await,
    }
}

async fn list(
    ticker: Option<String>,
    cik: Option<u64>,
    forms: Option<Vec<String>>,
    status: Option<FilingStatusArg>,
    limit: u32,
    cursor: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let api_key = config.require_api_key()?;
    let client = EarningsFeed::new(api_key)?;

    let mut builder = ListFilingsParams::builder().limit(limit);

    if let Some(t) = ticker {
        builder = builder.ticker(&t);
    }
    if let Some(c) = cik {
        builder = builder.cik(c);
    }
    if let Some(f) = forms {
        let form_refs: Vec<&str> = f.iter().map(|s| s.as_str()).collect();
        builder = builder.forms(form_refs);
    }
    if let Some(s) = status {
        let filing_status = match s {
            FilingStatusArg::All => FilingStatus::All,
            FilingStatusArg::Provisional => FilingStatus::Provisional,
            FilingStatusArg::Final => FilingStatus::Final,
        };
        builder = builder.status(filing_status);
    }
    if let Some(c) = cursor {
        builder = builder.cursor(&c);
    }

    let params = builder.build();
    let response = client.filings().list(&params).await?;

    let json = serde_json::to_string_pretty(&response)?;
    println!("{}", json);

    Ok(())
}

async fn get(accession: String) -> Result<()> {
    let config = Config::load()?;
    let api_key = config.require_api_key()?;
    let client = EarningsFeed::new(api_key)?;

    let filing = client.filings().get(&accession).await?;

    let json = serde_json::to_string_pretty(&filing)?;
    println!("{}", json);

    Ok(())
}
