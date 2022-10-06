use bybit_demo::rest_connector::{
    cancel_order, post_order, request_future_to_json, OrderType, Side,
};
use std::thread::sleep;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    //Part 1A: simple post order by bot
    let post_order_request = post_order("BTCUSDT", OrderType::Limit, 0.01, Some(10000.), Side::Buy);
    let post_order_response = request_future_to_json(post_order_request.await).await;

    println!(
        "Posted order, Order id :{:#?}",
        post_order_response.expect("Not get response")["result"]["orderId"]
    );

    // // Part 1B: Cancel Order API
    // let cancel_order_request = cancel_order("BTCUSDT", None);
    // let cancel_order_response = cancel_order_request
    //     .await
    //     .await
    //     .expect("Not get response")
    //     .status()
    //     .is_success();

    // match cancel_order_response {
    //     true => println!("Cancel Oredr success"),
    //     false => println!("Canceled failed"),
    // }
    // println!("");
    // //Part 2: apply Concurrency
    // let post_many_orders = vec![
    //     post_order("BTCUSDT", OrderType::Limit, 0.01, Some(10000.), Side::Buy),
    //     post_order("BTCUSDT", OrderType::Limit, 0.02, Some(10000.2), Side::Buy),
    //     post_order("BTCUSDT", OrderType::Limit, 0.03, Some(10000.3), Side::Buy),
    // ];

    // let mut handles = Vec::with_capacity(post_many_orders.len());

    // for order in post_many_orders {
    //     handles.push(tokio::spawn(order.await));
    // }

    // let mut results = Vec::with_capacity(handles.len());
    // for handle in handles {
    //     results.push(handle.await);
    // }

    // // Part 3
    // /*
    // Part 3 illustrates how the async improve the speed
    // 3.a is using sequential logic
    // 3.b is using Concurrency
    // */
    // // 3.a
    // let now = Instant::now();

    // {
    //     for _ in 0..10 {
    //         let post_order_request =
    //             post_order("BTCUSDT", OrderType::Limit, 0.01, Some(10000.), Side::Buy)
    //                 .await
    //                 .await;

    //         match post_order_request {
    //             Ok(_) => (),
    //             Err(e) => println!("{:}", e),
    //         }
    //     }
    // }
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
    // sleep(Duration::from_millis(2000));

    // // 3.b
    // let now = Instant::now();
    // {
    //     let mut post_many_orders = Vec::new();
    //     for i in 0..10 {
    //         post_many_orders.push(post_order(
    //             "BTCUSDT",
    //             OrderType::Limit,
    //             0.01 + i as f64,
    //             Some(10000. + i as f64),
    //             Side::Buy,
    //         ));
    //     }

    //     let mut handles = Vec::with_capacity(post_many_orders.len());

    //     for order in post_many_orders {
    //         handles.push(tokio::spawn(order.await));
    //     }

    //     let mut results = Vec::with_capacity(handles.len());
    //     for handle in handles {
    //         results.push(handle.await);
    //     }
    // }
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
