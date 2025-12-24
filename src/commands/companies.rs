//! Companies command implementation.

use earningsfeed::{EarningsFeed, SearchCompaniesParams};

use crate::cli::CompaniesAction;
use crate::config::Config;
use crate::error::Result;

pub async fn run(action: CompaniesAction) -> Result<()> {
    match action {
        CompaniesAction::Get { cik } => get(cik).await,
        CompaniesAction::Search {
            query,
            state,
            sic,
            limit,
        } => search(query, state, sic, limit).await,
    }
}

async fn get(cik: u64) -> Result<()> {
    let config = Config::load()?;
    let api_key = config.require_api_key()?;
    let client = EarningsFeed::new(api_key)?;

    let company = client.companies().get(cik).await?;

    let json = serde_json::to_string_pretty(&company)?;
    println!("{}", json);

    Ok(())
}

async fn search(query: String, state: Option<String>, sic: Option<u32>, limit: u32) -> Result<()> {
    let config = Config::load()?;
    let api_key = config.require_api_key()?;
    let client = EarningsFeed::new(api_key)?;

    let mut builder = SearchCompaniesParams::builder().q(&query).limit(limit);

    if let Some(s) = state {
        builder = builder.state(&s);
    }
    if let Some(s) = sic {
        builder = builder.sic_code(s);
    }

    let params = builder.build();
    let response = client.companies().search(&params).await?;

    let json = serde_json::to_string_pretty(&response)?;
    println!("{}", json);

    Ok(())
}
