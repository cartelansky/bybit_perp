use reqwest;
use serde_json::Value;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.bybit.com/v5/market/instruments-info?category=linear&status=Trading";
    let response = reqwest::get(url).await?.text().await?;
    let json: Value = serde_json::from_str(&response)?;

    let mut symbols: Vec<String> = Vec::new();

    if let Some(list) = json["result"]["list"].as_array() {
        for item in list {
            if let Some(symbol) = item["symbol"].as_str() {
                if symbol.ends_with("USDT") {
                    symbols.push(symbol.to_string());
                }
            }
        }
    }

    symbols.sort_by(|a, b| {
        let a_num = a
            .trim_end_matches(char::is_alphabetic)
            .parse::<u64>()
            .unwrap_or(0);
        let b_num = b
            .trim_end_matches(char::is_alphabetic)
            .parse::<u64>()
            .unwrap_or(0);

        match b_num.cmp(&a_num) {
            Ordering::Equal => a.cmp(b),
            other => other,
        }
    });

    let mut file = File::create("bybit_usdt_perpetual_futures.txt")?;

    for symbol in symbols {
        writeln!(file, "BYBIT:{}USDT.P", symbol.trim_end_matches("USDT"))?;
    }

    println!("İşlem tamamlandı. Sonuçlar 'bybit_usdt_perpetual_futures.txt' dosyasına yazıldı.");

    Ok(())
}
