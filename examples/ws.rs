use bybit::spot::ws::{PrivateResponse, PrivateWebSocketApiClient};

use bybit_demo::rest_connector::{post_order, OrderType, Side};
use futures::executor::block_on;

pub static API_KEY: &str = "";
pub static API_SECRET: &str = "";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = PrivateWebSocketApiClient::builder()
        .testnet()
        .build_with_credentials(&API_KEY, &API_SECRET);

    let callback = |res: PrivateResponse| match res {
        PrivateResponse::ExecutionReportSequence(seq) => {
            let _ = block_on(post_order(
                "BTCUSDT",
                OrderType::Limit,
                0.01,
                Some(10000.),
                Side::Buy,
            ));

            println!("Excution report: {:?}", seq)
        }
        PrivateResponse::TicketInfoSequence(seq) => println!("Ticket info: {:?}", seq),
        PrivateResponse::OutboundAccountInfoSequence(seq) => {
            println!("Outbound account info: {:?}", seq)
        }
        PrivateResponse::Pong(res) => println!("Pong: {:?}", res),
        PrivateResponse::Ping(res) => println!("Ping: {:?}", res),
    };

    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    Ok(())
}
