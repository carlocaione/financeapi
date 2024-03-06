use financeapi::FinanceapiConnector;

fn main() {
    // Here goes your API key
    let connector = FinanceapiConnector::new("...");

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
}
