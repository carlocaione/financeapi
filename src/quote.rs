use crate::FinanceapiError;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinanceapiQuote {
    pub language: String,
    pub region: String,
    pub quote_type: String,
    pub type_disp: Option<String>,
    pub quote_source_name: Option<String>,
    pub triggerable: bool,
    pub custom_price_alert_confidence: Option<String>,
    pub currency: Option<String>,
    pub market_state: String,
    pub regular_market_change_percent: Option<f64>,
    pub regular_market_price: Option<f64>,
    pub message_board_id: Option<String>,
    pub exchange: String,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub exchange_timezone_name: String,
    pub exchange_timezone_short_name: String,
    pub market: String,
    pub gmt_off_set_milliseconds: i32,
    pub esg_populated: bool,
    pub regular_market_previous_close: Option<f64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub bid_size: Option<i32>,
    pub ask_size: Option<i32>,
    pub full_exchange_name: String,
    pub financial_currency: Option<String>,
    pub regular_market_open: Option<f64>,
    pub average_daily_volume3_month: Option<u64>,
    pub average_daily_volume10_day: Option<u64>,
    pub fifty_two_week_low_change: Option<f64>,
    pub fifty_two_week_low_change_percent: Option<f64>,
    pub fifty_two_week_range: Option<String>,
    pub fifty_two_week_high_change: Option<f64>,
    pub fifty_two_week_high_change_percent: Option<f64>,
    pub fifty_two_week_low: Option<f64>,
    pub fifty_two_week_high: Option<f64>,
    pub dividend_date: Option<u64>,
    pub earnings_timestamp: Option<u64>,
    pub earnings_timestamp_start: Option<u64>,
    pub earnings_timestamp_end: Option<u64>,
    pub trailing_annual_dividend_rate: Option<f64>,
    pub trailing_p_e: Option<f64>,
    pub trailing_annual_dividend_yield: Option<f64>,
    pub eps_trailing_twelve_months: Option<f64>,
    pub eps_forward: Option<f64>,
    pub eps_current_year: Option<f64>,
    pub price_eps_current_year: Option<f64>,
    pub shares_outstanding: Option<u64>,
    pub book_value: Option<f64>,
    pub fifty_day_average: Option<f64>,
    pub fifty_day_average_change: Option<f64>,
    pub fifty_day_average_change_percent: Option<f64>,
    pub two_hundred_day_average: Option<f64>,
    pub two_hundred_day_average_change: Option<f64>,
    pub two_hundred_day_average_change_percent: Option<f64>,
    pub market_cap: Option<u64>,
    pub forward_p_e: Option<f64>,
    pub price_to_book: Option<f64>,
    pub source_interval: i32,
    pub exchange_data_delayed_by: Option<i32>,
    pub average_analyst_rating: Option<String>,
    pub tradeable: bool,
    pub crypto_tradeable: Option<bool>,
    pub regular_market_change: Option<f64>,
    pub regular_market_time: Option<u32>,
    pub regular_market_day_high: Option<f64>,
    pub regular_market_day_range: Option<String>,
    pub regular_market_day_low: Option<f64>,
    pub regular_market_volume: Option<u64>,
    pub first_trade_date_milliseconds: Option<i64>,
    pub price_hint: i32,
    pub display_name: Option<String>,
    pub symbol: String,
    pub coin_image_url: Option<String>,
    pub logo_url: Option<String>,
    pub circulating_supply: Option<u64>,
    pub last_market: Option<String>,
    pub volume24_hr: Option<u64>,
    pub volume_all_currencies: Option<u64>,
    pub from_currency: Option<String>,
    pub to_currency: Option<String>,
    pub coin_market_cap_link: Option<String>,
    pub start_date: Option<u32>,
}

#[derive(Deserialize, Debug)]
struct QuoteResponse {
    result: Vec<FinanceapiQuote>,
    error: Option<String>,
}

impl FinanceapiQuote {
    /// Build a `FinanceapiQuote` object from JSON
    pub fn from_json(json: serde_json::Value) -> Result<FinanceapiQuote, FinanceapiError> {
        let mut qr: QuoteResponse = serde_json::from_value(
            json.get("quoteResponse")
                .ok_or(FinanceapiError::JsonParseError)?
                .to_owned(),
        )?;

        if let Some(err) = qr.error {
            return Err(FinanceapiError::JsonResponseError(err));
        }

        if qr.result.is_empty() {
            return Err(FinanceapiError::SymbolNotFoundError);
        }

        qr.result.pop().ok_or(FinanceapiError::UnexpectedError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    #[should_panic(expected = "JsonParseError")]
    fn wrong_json() {
        let j = json!({ "var": 200 });
        FinanceapiQuote::from_json(j).unwrap();
    }

    #[test]
    #[should_panic(expected = "foobar")]
    fn wrong_json_response() {
        let j = json!({
          "quoteResponse": {
            "result": [],
            "error": "foobar"
          }
        });
        FinanceapiQuote::from_json(j).unwrap();
    }

    #[test]
    #[should_panic(expected = "SymbolNotFoundError")]
    fn wrong_symbol() {
        let j = json!({
          "quoteResponse": {
            "result": [],
            "error": null,
          }
        });
        FinanceapiQuote::from_json(j).unwrap();
    }

    #[test]
    #[should_panic(expected = "JsonSerdeError")]
    fn wrong_json_serde() {
        let j = json!({
          "quoteResponse": {
            "result": [
              {
                "symbol": "AAPL"
              }
            ],
            "error": null
          }
        });
        FinanceapiQuote::from_json(j).unwrap();
    }

    #[test]
    fn check_parsing() {
        let j = json!({
          "quoteResponse": {
            "result": [
              {
                "language": "en-US",
                "region": "US",
                "quoteType": "EQUITY",
                "triggerable": true,
                "marketState": "REGULAR",
                "fullExchangeName": "NasdaqGS",
                "exchangeTimezoneName": "Europe/Rome",
                "exchangeTimezoneShortName": "CET",
                "sourceInterval": 15,
                "gmtOffSetMilliseconds": -18000000,
                "esgPopulated": false,
                "tradeable": false,
                "exchange": "NMS",
                "priceHint": 2,
                "market": "us_market",
                "symbol": "AAPL"
              }
            ],
            "error": null
          }
        });

        let quote = FinanceapiQuote::from_json(j).unwrap();
        assert_eq!(quote.language, "en-US");
        assert_eq!(quote.symbol, "AAPL");
        assert_eq!(quote.source_interval, 15);
        assert!(!quote.tradeable);
    }
}
