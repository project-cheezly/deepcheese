use crate::client::cheese_api::FutureCurrentPriceResponse;
use crate::model::collector::response::StreamSerializer;

#[derive(Default, serde::Serialize)]
pub struct FuturePrice {
    time: i32,
    price: i32,
    amount: i32,
    bid_price:i32,
    ask_price: i32
}

impl From<FutureCurrentPriceResponse> for FuturePrice {
    fn from(value: FutureCurrentPriceResponse) -> Self {
        FuturePrice {
            time: value.time,
            price: value.price,
            amount: value.amount,
            bid_price: value.bid_price,
            ask_price: value.ask_price,
        }
    }
}

impl StreamSerializer for FuturePrice {
    type Input = FutureCurrentPriceResponse;

    fn get_headers() -> Vec<String> {
        vec![
            "time".to_string(),
            "price".to_string(),
            "amount".to_string(),
            "bid_price".to_string(),
            "ask_price".to_string(),
        ]
    }
}