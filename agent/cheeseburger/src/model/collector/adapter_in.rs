use tonic::Streaming;
use tonic::transport::Channel;
use crate::client::cheese_api::{
    FutureCurrentPriceRequest,
    FutureCurrentPriceResponse,
    FutureLimitOrderBookRequest,
    FutureLimitOrderBookResponse
};
use crate::client::cheese_api::cheese_api_client::CheeseApiClient;

pub async fn get_limit_order_book_stream(client: &mut CheeseApiClient<Channel>, code: &str)
                                         -> Result<Streaming<FutureLimitOrderBookResponse>, Box<dyn std::error::Error + Send + Sync>>
{
    Ok(client.lookup_future_limit_order_book_realtime(
        FutureLimitOrderBookRequest {
            code: code.to_string(),
        })
        .await?
        .into_inner())
}

pub async fn get_future_price_stream(client: &mut CheeseApiClient<Channel>, code: &str)
    -> Result<Streaming<FutureCurrentPriceResponse>, Box<dyn std::error::Error + Send + Sync>>
{
    Ok(client.lookup_future_current_price_realtime(
        FutureCurrentPriceRequest {
            code: code.to_string(),
        })
        .await?
        .into_inner())
}
