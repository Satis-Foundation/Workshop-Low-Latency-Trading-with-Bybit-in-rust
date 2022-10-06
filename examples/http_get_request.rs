use bybit_demo::rest_connector::response_to_json;

const TESTNET: &str = "https://api-testnet.bybit.com/spot/v3";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    // // Part 1. Simple GET request
    let get_all_symbols_path = String::from("/public/quote/ticker/24hr");
    let request1 = client
        .get(format!("{TESTNET}{get_all_symbols_path}"))
        .query(&[("symbol", "BTCUSDT")])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("simple GET request response: {:#?}", request1["result"]);

    // // Part 2. Async GET request
    // let async_request1 = client
    //     .get(format!("{TESTNET}{get_all_symbols_path}"))
    //     .query(&[("symbol", "BTCUSDT")])
    //     .send();

    // let async_request2 = client
    //     .get(format!("{TESTNET}{get_all_symbols_path}"))
    //     .query(&[("symbol", "ETHUSDT")])
    //     .send();

    // let (r1, _r2) = tokio::join!(async_request1, async_request2);
    // let r1_json = response_to_json(r1);

    // println!(
    //     "Part 2, {:#?}",
    //     r1_json.await.expect("Can't get tge r1 response")["result"]
    // );

    Ok(())
}
