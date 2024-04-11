pub mod limit_order_book;
pub mod future_price;

use serde::Serialize;
pub use limit_order_book::LimitOrderBook;
pub use future_price::FuturePrice;

pub trait StreamSerializer: From<Self::Input> + Serialize {
    type Input;

    fn get_headers() -> Vec<String>;
}