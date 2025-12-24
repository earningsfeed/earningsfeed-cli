//! Institutional command implementation.

use earningsfeed::{EarningsFeed, ListInstitutionalParams, PutCallFilter};

use crate::cli::{InstitutionalAction, PutCallArg};
use crate::config::Config;
use crate::error::Result;

pub async fn run(action: InstitutionalAction) -> Result<()> {
    match action {
        InstitutionalAction::List {
            ticker,
            cik,
            manager_cik,
            put_call,
            limit,
            cursor,
        } => list(ticker, cik, manager_cik, put_call, limit, cursor).await,
    }
}

async fn list(
    ticker: Option<String>,
    cik: Option<u64>,
    manager_cik: Option<u64>,
    put_call: Option<PutCallArg>,
    limit: u32,
    cursor: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let api_key = config.require_api_key()?;
    let client = EarningsFeed::new(api_key)?;

    let mut builder = ListInstitutionalParams::builder().limit(limit);

    if let Some(t) = ticker {
        builder = builder.ticker(&t);
    }
    if let Some(c) = cik {
        builder = builder.cik(c);
    }
    if let Some(mc) = manager_cik {
        builder = builder.manager_cik(mc);
    }
    if let Some(pc) = put_call {
        let filter = match pc {
            PutCallArg::Put => PutCallFilter::Put,
            PutCallArg::Call => PutCallFilter::Call,
            PutCallArg::Equity => PutCallFilter::Equity,
        };
        builder = builder.put_call(filter);
    }
    if let Some(c) = cursor {
        builder = builder.cursor(&c);
    }

    let params = builder.build();
    let response = client.institutional().list(&params).await?;

    let json = serde_json::to_string_pretty(&response)?;
    println!("{}", json);

    Ok(())
}
