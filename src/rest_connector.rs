use futures::Future;
use hex;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::{Error, Response};
use ring::hmac;
use serde_json::{json, Value};
use std::time::SystemTime;

const TESTNET: &str = "https://api-testnet.bybit.com/spot/v3";

pub static API_KEY: &str = "";
pub static API_SECRET: &str = "";

pub enum OrderType {
    Limit,
    Market,
}

pub enum Side {
    Buy,
    Sell,
}

impl Side {
    pub fn to_string(&self) -> &str {
        match *self {
            Side::Buy => "Buy",
            Side::Sell => "Sell",
        }
    }
}

// for auth
pub fn get_current_time() -> Result<u128, std::time::SystemTimeError> {
    let d = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(d.as_millis())
}

pub fn sign(secret: &str, msg: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let tag = hmac::sign(&key, msg.as_bytes());
    hex::encode(tag.as_ref())
}

pub fn construct_private_headers(signature: &str, timestamp: u128) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("X-BAPI-API-KEY", HeaderValue::from_static(API_KEY));
    headers.insert("X-BAPI-SIGN", HeaderValue::from_str(signature).unwrap());
    headers.insert("X-BAPI-SIGN-TYPE", HeaderValue::from_static("2"));
    headers.insert(
        "X-BAPI-TIMESTAMP",
        HeaderValue::from_str(&timestamp.to_string()).expect("Invalid timestamp"),
    );
    headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from_static("5000"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    headers
}

pub async fn rest_private_api_ap(
    path: &str,
    json_input: &Value,
) -> impl Future<Output = Result<Response, Error>> {
    let time = get_current_time().expect("Invalid expires");

    let val = format!(
        "{time}{api_key}5000{params}",
        time = time,
        api_key = API_KEY,
        params = json_input.to_string(),
    );

    let signature = sign(&API_SECRET, &val);

    let request = reqwest::Client::new()
        .post(format!("{TESTNET}{path}"))
        .json(&json!(json_input))
        .headers(construct_private_headers(&signature, time))
        .send();
    request
}

pub async fn post_order(
    symbol: &str,
    order_type: OrderType,
    size: f64,
    price: Option<f64>,
    side: Side,
) -> impl Future<Output = Result<Response, Error>> {
    let params = match order_type {
        OrderType::Limit => match price {
            Some(price) => Ok(
                json!({"orderPrice": price.to_string(), "orderQty": size.to_string(), "orderType": "Limit", "side": side.to_string(), "symbol": symbol, "timeInForce": "GTC"}),
            ),
            None => Err("No price for limit order"),
        },
        _ => match price {
            Some(_price) => Err("Price should not in market order"),
            None => Ok(
                json!({"orderQty": size.to_string(), "orderType": "Market", "side": side.to_string(), "symbol": symbol, "timeInForce": "GTC"}),
            ),
        },
    };
    rest_private_api_ap("/private/order", &params.expect("Wrong input")).await
}

pub async fn cancel_order(
    symbol: &str,
    order_type: Option<OrderType>,
) -> impl Future<Output = Result<Response, Error>> {
    let params = match order_type {
        Some(x) => match x {
            OrderType::Limit => json!({"symbol": symbol, "orderTypes":"Limit"}),
            OrderType::Market => json!({"symbol": symbol, "orderTypes":"Limit"}),
        },
        None => json!({ "symbol": symbol }),
    };
    rest_private_api_ap("/private/cancel-orders", &params).await
}

pub async fn request_future_to_json(
    response: impl Future<Output = Result<Response, Error>>,
) -> Result<serde_json::Value, Error> {
    Ok(response.await?.json::<serde_json::Value>().await?)
}

pub async fn response_to_json(
    response: Result<Response, Error>,
) -> Result<serde_json::Value, Error> {
    Ok(response?.json::<serde_json::Value>().await?)
}
