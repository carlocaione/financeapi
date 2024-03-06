use crate::FinanceapiError;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinanceapiAutocomplete {
    pub symbol: String,
    pub name: String,
    pub exch: String,
    #[serde(rename(deserialize = "type"))]
    pub exch_type: String,
    pub exch_disp: String,
    pub type_disp: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ResultSet {
    _query: Option<String>,
    result: Vec<FinanceapiAutocomplete>,
}

impl FinanceapiAutocomplete {
    /// Build a `FinanceapiAutocomplete` object from JSON
    pub fn from_json(
        json: serde_json::Value,
    ) -> Result<Vec<FinanceapiAutocomplete>, FinanceapiError> {
        let rs: ResultSet = serde_json::from_value(
            json.get("ResultSet")
                .ok_or(FinanceapiError::JsonParseError)?
                .to_owned(),
        )?;

        if rs.result.is_empty() {
            return Err(FinanceapiError::SymbolNotFoundError);
        }

        Ok(rs.result)
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
        FinanceapiAutocomplete::from_json(j).unwrap();
    }

    #[test]
    #[should_panic(expected = "SymbolNotFoundError")]
    fn wrong_symbol() {
        let j = json!({
          "ResultSet": {
            "Query": "unknown",
            "Result": []
          }
        });
        FinanceapiAutocomplete::from_json(j).unwrap();
    }

    #[test]
    #[should_panic(expected = "JsonSerdeError")]
    fn wrong_json_serde() {
        let j = json!({
          "ResultSet": {
            "Query": "VWCE.MI",
            "Result": [
              {
                "symbol": "VWCE.MI",
              }
            ]
          }
        });
        FinanceapiAutocomplete::from_json(j).unwrap();
    }

    #[test]
    fn check_parsing() {
        let j = json!({
          "ResultSet": {
            "Query": "VWCE.MI",
            "Result": [
              {
                "symbol": "VWCE.MI",
                "name": "Vanguard FTSE All-World UCITS ETF USD Accumulation",
                "exch": "MIL",
                "type": "E",
                "exchDisp": "Milan",
                "typeDisp": "ETF"
              }
            ]
          }
        });

        let vq = FinanceapiAutocomplete::from_json(j).unwrap();
        assert_eq!(vq.len(), 1);

        let query = &vq[0];
        assert_eq!(query.symbol, "VWCE.MI");
        assert_eq!(query.exch, "MIL");
    }
}
