use crate::client::cheese_api::FutureLimitOrderBookResponse;
use crate::model::collector::response::StreamSerializer;

#[derive(Default, serde::Serialize)]
pub struct LimitOrderBook {
    time: i32,
    bid: Vec<LimitOrder>,
    ask: Vec<LimitOrder>,
    expected_price: i32
}

#[derive(Default, serde::Serialize)]
struct LimitOrder {
    price: i32,
    amount: i32,
    count: i32
}

impl From<FutureLimitOrderBookResponse> for LimitOrderBook {
    fn from(value: FutureLimitOrderBookResponse) -> Self {
        LimitOrderBook {
            time: value.time,
            bid: value.bid.into_iter().map(|order| LimitOrder {
                price: order.price,
                amount: order.amount,
                count: order.count.unwrap_or_else(|| 0),
            }).collect(),
            ask: value.ask.into_iter().map(|order| LimitOrder {
                price: order.price,
                amount: order.amount,
                count: order.count.unwrap_or_else(|| 0),
            }).collect(),
            expected_price: value.expected_price,
        }
    }
}

impl StreamSerializer for LimitOrderBook {
    type Input = FutureLimitOrderBookResponse;

    fn get_headers() -> Vec<String> {
        let mut record = vec![String::from("time")];

        for side in ["bid", "ask"] {
            for idx in 0..5 {
                for field in ["price", "amount", "count"] {
                    record.push(format!("{}_{}_{}", side, idx + 1, field));
                }
            }
        }

        record.push(String::from("expected_price"));
        record
    }
}
