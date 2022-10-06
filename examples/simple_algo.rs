use bybit::spot::ws::{PublicResponse, PublicWebSocketApiClient};

use bybit_demo::rest_connector::{post_order, OrderType, Side};
use futures::executor::block_on;
use std::collections::VecDeque;

pub static API_KEY: &str = "";
pub static API_SECRET: &str = "";

#[derive(Debug, PartialEq)]
enum Direction {
    UP,
    DOWN,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut client = PublicWebSocketApiClient::new();

    client.subscribe_trade("BTCUSDT", false);

    let mut dir_history_vec = VecDeque::<Direction>::with_capacity(5);
    let mut previous_price: f64 = 0.;

    let callback = |res: PublicResponse| match res {
        PublicResponse::Trade(res) => {
            let trade = &res.data[0];
            let last_price: f64 = trade.p.parse().unwrap();
            let last_quant: f64 = trade.q.parse().unwrap();

            println!("{:}", last_price);
            println!("{:}", last_quant);

            if last_price > previous_price {
                dir_history_vec.push_back(Direction::UP);
            } else {
                dir_history_vec.push_back(Direction::DOWN);
            }
            previous_price = last_price;
            dir_history_vec.pop_front();

            let mut dir_count = 0;
            for i in dir_history_vec.iter() {
                if i == &dir_history_vec[0] {
                    dir_count += 1;
                }
            }

            if dir_count == 5 {
                // Post Order
                let (order_price, order_side) = match dir_history_vec.pop_back().unwrap() {
                    Direction::UP => (last_price + 1., Side::Buy),
                    Direction::DOWN => (last_price - 1., Side::Sell),
                };

                let _ = block_on(post_order(
                    "BTCUSDT",
                    OrderType::Limit,
                    last_quant,
                    Some(order_price),
                    order_side,
                ));
            }
        }
        _ => (),
    };
    match client.run(callback) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    Ok(())
}
