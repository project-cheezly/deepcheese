use chrono::{Local, NaiveTime};
use log::{error, info};
use tokio::task::JoinHandle;
use tonic::Streaming;
use tonic::transport::Channel;
use crate::client;
use crate::client::cheese_api::cheese_api_client::CheeseApiClient;
use crate::error::CheeseburgerError;
use crate::model::collector::adapter_out::{CsvCollector, DataCollector};
use crate::model::query_code::QueryCode;
use crate::model::collector::adapter_in::{
    get_future_price_stream,
    get_limit_order_book_stream
};
use crate::model::collector::response::{
    FuturePrice,
    LimitOrderBook,
    StreamSerializer
};

const END_UP_TIME: Option<NaiveTime> = NaiveTime::from_hms_opt(15, 50, 0);

pub struct TargetData {
    pub query_code: QueryCode,
    pub stock_code: String,
}

pub async fn start_collector_service(targets: Vec<TargetData>)
                                     -> Result<(), Box<dyn std::error::Error>>
{
    let mut client = client::new().await
        .or_else(|e| {
            error!("Failed to connect to Cheese API: {}", e);
            Err(CheeseburgerError::ConnectionError(e.to_string()))
        })?;

    let mut task_handles = vec![];
    for target in targets {
        info!("Start collecting {} data for stock code: {}", &target.query_code, &target.stock_code);

        let handle = match target.query_code {
            QueryCode::FutureOptionCurrentPrice => { start_future_price_collector_service(&mut client, &target.stock_code).await },
            QueryCode::FutureOptionLimitOrderBook => { start_future_limit_order_book_collector_service(&mut client, &target.stock_code).await }
        };

        if let Ok(handle) = handle {
            task_handles.push(handle);
        } else {
            error!("Failed to start collector service for stock code: {}", &target.stock_code);
        }
    }

    end_up_service(task_handles);

    Ok(())
}

fn end_up_service(task_handles: Vec<JoinHandle<()>>) {
    tokio::spawn(async move {
        let now = Local::now();
        let duration = (now.with_time(END_UP_TIME.unwrap()).unwrap() - now)
            .to_std()
            .unwrap_or_default();

        tokio::time::sleep(duration).await;

        for handle in task_handles {
            handle.abort();
        }
    });
}

async fn start_future_price_collector_service(client: &mut CheeseApiClient<Channel>, stock_code: &str)
    -> Result<JoinHandle<()>, Box<dyn std::error::Error>>
{
    let csv_writer = CsvCollector::<FuturePrice>::new(&QueryCode::FutureOptionCurrentPrice.to_string(), stock_code).unwrap();

    if let Ok(stream) = get_future_price_stream(client, stock_code).await {
        Ok(tokio::spawn(data_loop(csv_writer, stream)))
    } else {
        Err(Box::new(CheeseburgerError::StreamError))
    }
}

async fn start_future_limit_order_book_collector_service(client: &mut CheeseApiClient<Channel>, stock_code: &str)
    -> Result<JoinHandle<()>, Box<dyn std::error::Error>>
{
    let csv_writer = CsvCollector::<LimitOrderBook>::new(&QueryCode::FutureOptionLimitOrderBook.to_string(), stock_code).unwrap();

    if let Ok(stream) = get_limit_order_book_stream(client, stock_code).await {
        Ok(tokio::spawn(data_loop(csv_writer, stream)))
    } else {
        Err(Box::new(CheeseburgerError::StreamError))
    }
}

async fn data_loop<T, V>(mut writer: V, mut stream: Streaming<T::Input>)
    where T: StreamSerializer,
          V: DataCollector<T>
{
    while let Ok(Some(data)) = stream.message().await {
        match writer.write(data) {
            Ok(_) => {},
            Err(e) => { error!("Failed to write data: {}", e); }
        }
    }
}