use std::collections::HashMap;
use tokio::sync::broadcast::{Receiver, Sender};
use tonic::Streaming;
use crate::client;
use crate::client::cheese_api::{
    FutureCurrentPriceRequest,
    FutureCurrentPriceResponse,
    FutureLimitOrderBookRequest,
    FutureLimitOrderBookResponse
};

const MAX_BUFFER_SIZE: usize = 50;

#[derive(Debug, Default)]
pub struct StreamManager {
    future_price: HashMap<String, Receiver<FutureCurrentPriceResponse>>,
    future_limit_order_book: HashMap<String, Receiver<FutureLimitOrderBookResponse>>
}

impl StreamManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn get_future_price_receiver(&mut self, code: &str)
        -> Receiver<FutureCurrentPriceResponse>
    {
        if !self.future_price.contains_key(code) {
            let (tx, rx) = tokio::sync::broadcast::channel(MAX_BUFFER_SIZE);

            tokio::spawn(get_future_price_stream(tx, code.to_string()));
            self.future_price.insert(code.to_string(), rx);
        }

        self.future_price.get(code).unwrap().resubscribe()
    }

    pub async fn get_future_limit_order_book_stream(&mut self, code: &str)
        -> Receiver<FutureLimitOrderBookResponse>
    {
        if !self.future_limit_order_book.contains_key(code) {
            let (tx, rx) = tokio::sync::broadcast::channel(MAX_BUFFER_SIZE);

            tokio::spawn(get_limit_order_book_stream(tx, code.to_string()));
            self.future_limit_order_book.insert(code.to_string(), rx);
        }

        self.future_limit_order_book.get(code).unwrap().resubscribe()
    }
}

async fn get_future_price_stream(tx: Sender<FutureCurrentPriceResponse>, code: String)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let mut client = client::new().await?;
    let stream = client.lookup_future_current_price_realtime(
        FutureCurrentPriceRequest {
            code: code,
        }
    ).await?.into_inner();

    message_loop(stream, tx).await;

    Ok(())
}

async fn get_limit_order_book_stream(tx: Sender<FutureLimitOrderBookResponse>, code: String)
    -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let mut client = client::new().await?;
    let stream = client.lookup_future_limit_order_book_realtime(
        FutureLimitOrderBookRequest {
            code,
        }
    ).await?.into_inner();

    message_loop(stream, tx).await;

    Ok(())
}

async fn message_loop<T>(mut stream: Streaming<T>, tx: Sender<T>) {
    let mut started = false;

    while let Ok(Some(response)) = stream.message().await {
        let res = tx.send(response);

        if let Ok(res) = res {
            if !started && res > 0 { started = true; }
            else if started && res == 0 { break; }
        }
    }
}