//! # financeapi
//!
//! This crate provides a simple set of APIs to interface with
//! [financeapi.net](http://financeapi.net) to retrieve financial data for
//! stocks, ETFs, mutual funds, etc...
//!
//! To be able to use this API the user needs to register and get an API key
//! from [financeapi.net](http://financeapi.net).
//!
//! Currently only the following modules are available:
//! - `/v6/finance/quote` (Real time quote data for stocks, ETFs, mutuals funds, etc...)
//! - `/v6/finance/autocomplete` (Get auto complete stock suggestions)
//!
//! The crate is using `reqwest` with `async` features. In a blocking /
//! synchronous context these functions must be called using `block_on` or
//! equivalent (see example).
//!
//! # Examples
//!
//! ```ignore
//!    
//!     // Here goes your API key
//!     let connector = FinanceapiConnector::new("...");
//!
//!     // v6/finance/quote
//!     let quote = tokio::runtime::Builder::new_current_thread()
//!         .enable_all()
//!         .build()
//!         .unwrap()
//!         .block_on(connector.quote("AAPL"))
//!         .unwrap_or_else(|e| panic!("ERROR: {}", e));
//!
//!     println!(
//!         "AAPL ({}) is currently at {} {}",
//!         quote.long_name.unwrap_or_default(),
//!         quote.regular_market_price.unwrap_or_default(),
//!         quote.financial_currency.unwrap_or_default()
//!     );
//!
//!     let symbol = "VWCE";
//!
//!     // v6/finance/autocomplete
//!     let search = tokio::runtime::Builder::new_current_thread()
//!         .enable_all()
//!         .build()
//!         .unwrap()
//!         .block_on(connector.autocomplete(symbol))
//!         .unwrap_or_else(|e| panic!("ERROR: {}", e));
//!
//!     println!("\nFound {} results for {}", search.len(), symbol);
//!
//!     for (i, v) in search.iter().enumerate() {
//!         println!("{}: {} ({})", i, v.symbol, v.name);
//!     }
//! ```

use const_format::formatcp;
use reqwest::IntoUrl;
use reqwest::{Client, Response};
use url::Url;

mod autocomplete;
mod error;
mod quote;

pub use autocomplete::FinanceapiAutocomplete;
pub use error::FinanceapiError;
pub use quote::FinanceapiQuote;

const YH_LOCALE: &str = "region=US&lang=en";
const YH_URL: &str = "https://yfapi.net";

const YH_MODULE_FQ: &str = "v6/finance/quote";
const YH_URL_FQ: &str = formatcp!("{YH_URL}/{YH_MODULE_FQ}?{YH_LOCALE}");

const YH_MODULE_FA: &str = "v6/finance/autocomplete";
const YH_URL_FA: &str = formatcp!("{YH_URL}/{YH_MODULE_FA}?{YH_LOCALE}");

#[derive(Default, Debug)]
pub struct FinanceapiConnector {
    client: Client,
    api_key: String,
}

impl FinanceapiConnector {
    async fn send_request<U: IntoUrl>(&self, url: U) -> Result<Response, FinanceapiError> {
        Ok(self
            .client
            .get(url)
            .header("accept", "application/json")
            .header("X-API-KEY", &self.api_key)
            .send()
            .await?
            .error_for_status()?)
    }

    async fn json<U: IntoUrl>(&self, url: U) -> Result<serde_json::Value, FinanceapiError> {
        Ok(self.send_request(url).await?.json().await?)
    }

    /// Create a new connection to [financeapi.net](http://financeapi.net) using the
    /// provided API key.
    ///
    /// To get an API key refer to [financeapi.net](http://financeapi.net).
    pub fn new<T: Into<String>>(key: T) -> Self {
        Self {
            client: Client::new(),
            api_key: key.into(),
        }
    }

    /// Get real time quote data for stocks, ETFs, mutuals funds, etc... given a
    /// provided symbol.
    ///
    /// This is leveraging the `/v6/finance/quote` module.
    pub async fn quote<S: AsRef<str>>(
        &self,
        symbol: S,
    ) -> Result<FinanceapiQuote, FinanceapiError> {
        let url = Url::parse_with_params(YH_URL_FQ, &[("symbols", symbol.as_ref())])?;
        let json = self.json(url).await?;

        FinanceapiQuote::from_json(json)
    }

    /// Get autocomplete stock suggestions given a provided string.
    ///
    /// This is leveraging the `/v6/finance/autocomplete` module.
    pub async fn autocomplete<S: AsRef<str>>(
        &self,
        query: S,
    ) -> Result<Vec<FinanceapiAutocomplete>, FinanceapiError> {
        let url = Url::parse_with_params(YH_URL_FA, &[("query", query.as_ref())])?;
        let json = self.json(url).await?;

        FinanceapiAutocomplete::from_json(json)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[should_panic(expected = "403")]
    async fn wrong_api_key() {
        use super::*;

        let connector = FinanceapiConnector::new("ABC");
        connector.quote("SYMBOL").await.unwrap();
    }
}
