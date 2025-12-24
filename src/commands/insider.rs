//! Insider command implementation.

use earningsfeed::{EarningsFeed, ListInsiderParams, TransactionDirection};

use crate::cli::{InsiderAction, TransactionDirectionArg};
use crate::config::Config;
use crate::error::Result;

pub async fn run(action: InsiderAction) -> Result<()> {
    match action {
        InsiderAction::List {
            ticker,
            cik,
            person_cik,
            direction,
            min_value,
            limit,
            cursor,
        } => list(ticker, cik, person_cik, direction, min_value, limit, cursor).await,
    }
}

async fn list(
    ticker: Option<String>,
    cik: Option<u64>,
    person_cik: Option<u64>,
    direction: Option<TransactionDirectionArg>,
    min_value: Option<u64>,
    limit: u32,
    cursor: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let api_key = config.require_api_key()?;
    let client = EarningsFeed::new(api_key)?;

    let mut builder = ListInsiderParams::builder().limit(limit);

    if let Some(t) = ticker {
        builder = builder.ticker(&t);
    }
    if let Some(c) = cik {
        builder = builder.cik(c);
    }
    if let Some(pc) = person_cik {
        builder = builder.person_cik(pc);
    }
    if let Some(d) = direction {
        let dir = match d {
            TransactionDirectionArg::Buy => TransactionDirection::Buy,
            TransactionDirectionArg::Sell => TransactionDirection::Sell,
        };
        builder = builder.direction(dir);
    }
    if let Some(mv) = min_value {
        builder = builder.min_value(mv);
    }
    if let Some(c) = cursor {
        builder = builder.cursor(&c);
    }

    let params = builder.build();
    let response = client.insider().list(&params).await?;

    let json = serde_json::to_string_pretty(&response)?;
    println!("{}", json);

    Ok(())
}
