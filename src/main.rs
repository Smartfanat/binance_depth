use std::io::{self, Write};
use serde::Deserialize;
use reqwest::blocking::get;

#[derive(Debug, Deserialize)]
struct OrderBook {
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

fn fetch_order_book(pair: &str) -> Result<OrderBook, Box<dyn std::error::Error>> {
    let url = format!("https://api.binance.com/api/v3/depth?symbol={}&limit=5", pair);
    let res = get(&url)?;
    let book: OrderBook = res.json()?;
    Ok(book)
}

fn print_order_book(pair: &str, book: &OrderBook) {
    println!("\n--- Top 10 Order Book for {} ---", pair);

    println!("\nAsks (Sell Orders):");
    for ask in &book.asks {
        println!("Price: {:>12} | Qty: {}", ask[0], ask[1]);
    }

    println!("\nBids (Buy Orders):");
    for bid in &book.bids {
        println!("Price: {:>12} | Qty: {}", bid[0], bid[1]);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pairs = ["BTCUSDT", "ETHUSDT", "BNBUSDT", "XRPUSDT", "SOLUSDT"];

    println!("Select a trading pair:");
    for (i, pair) in pairs.iter().enumerate() {
        println!("{}. {}", i + 1, pair);
    }

    print!("Enter choice (1-5): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(0);

    if choice == 0 || choice > pairs.len() {
        println!("Invalid selection.");
        return Ok(());
    }

    let selected_pair = pairs[choice - 1];
    let book = fetch_order_book(selected_pair)?;
    print_order_book(selected_pair, &book);

    Ok(())
}
