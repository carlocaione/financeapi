# financeapi

This crate provides a simple set of APIs to interface with
[financeapi.net](http://financeapi.net) to retrieve financial data for
stocks, ETFs, mutual funds, etc...

To be able to use this API the user needs to register and get an API key
from [financeapi.net](http://financeapi.net).

Currently only the following modules are available:
- `/v6/finance/quote` (Real time quote data for stocks, ETFs, mutuals funds, etc...)
- `/v6/finance/autocomplete` (Get auto complete stock suggestions)

The crate is using `reqwest` with `async` features. In a blocking /
synchronous context these functions must be called using `block_on` or
equivalent (see example).

# Examples

```rust
   
    // Here goes your API key
    let connector = FinanceapiConnector::new("...");

    // v6/finance/quote
    let quote = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(connector.quote("AAPL"))
        .unwrap_or_else(|e| panic!("ERROR: {}", e));

    println!(
        "AAPL ({}) is currently at {} {}",
        quote.long_name.unwrap_or_default(),
        quote.regular_market_price.unwrap_or_default(),
        quote.financial_currency.unwrap_or_default()
    );

    let symbol = "VWCE";

    // v6/finance/autocomplete
    let search = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(connector.autocomplete(symbol))
        .unwrap_or_else(|e| panic!("ERROR: {}", e));

    println!("\nFound {} results for {}", search.len(), symbol);

    for (i, v) in search.iter().enumerate() {
        println!("{}: {} ({})", i, v.symbol, v.name);
    }
```
