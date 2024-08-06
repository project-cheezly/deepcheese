use std::sync::Arc;
use chrono::{Local, NaiveTime};
use tracing::{error, info};
use tokio::sync::broadcast::Receiver;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use crate::core::{
    future::stream::StreamManager,
    indi::QueryCode
};
use crate::model::collector::{
    adapter_out::{CsvCollector, DataCollector},
    response::{FuturePrice, LimitOrderBook, StreamSerializer},
    config
};

pub async fn start_collector_service(
    stream_manager: Arc<Mutex<StreamManager>>
) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let config = config::load().await?;

    let mut task_handles = vec![];

    for target in config.target {
        info!(
            "Start collecting {} data for stock code: {}",
            &target.query_code,
            &target.stock_code
        );

        let handle = match target.query_code {
            QueryCode::FutureOptionCurrentPrice =>
                start_future_price_collector_service(stream_manager.clone(), &target.stock_code).await,
            QueryCode::FutureOptionLimitOrderBook =>
                start_future_limit_order_book_collector_service(stream_manager.clone(), &target.stock_code).await
        };

        if let Ok(handle) = handle {
            task_handles.push(handle);
        } else {
            error!("Failed to start collector service for stock code: {}", &target.stock_code);
        }
    }

    end_up_service(task_handles, config.end_time);

    Ok(())
}

fn end_up_service(
    task_handles: Vec<JoinHandle<()>>,
    end_up_time: NaiveTime
) {
    tokio::spawn(async move {
        let now = Local::now();
        let duration = (now.with_time(end_up_time).unwrap() - now)
            .to_std()
            .unwrap_or_default();

        tokio::time::sleep(duration).await;

        for handle in task_handles {
            handle.abort();
        }
    });
}

async fn start_future_price_collector_service(
    stream_manager: Arc<Mutex<StreamManager>>,
    code: &str
) -> Result<JoinHandle<()>, Box<dyn std::error::Error + Sync + Send>>
{
    let csv_writer =
        CsvCollector::<FuturePrice>::new(
            &QueryCode::FutureOptionCurrentPrice.to_string(),
            code
        )?;

    let stream = stream_manager
        .lock()
        .await
        .get_future_price_receiver(code)
        .await;

    Ok(tokio::spawn(data_loop(csv_writer, stream)))
}

async fn start_future_limit_order_book_collector_service(
    stream_manager: Arc<Mutex<StreamManager>>,
    code: &str
) -> Result<JoinHandle<()>, Box<dyn std::error::Error + Sync + Send>>
{
    let csv_writer =
        CsvCollector::<LimitOrderBook>::new(
            &QueryCode::FutureOptionLimitOrderBook.to_string(),
            code
        )?;

    let stream = stream_manager
        .lock()
        .await
        .get_future_limit_order_book_stream(code)
        .await;

    Ok(tokio::spawn(data_loop(csv_writer, stream)))
}

async fn data_loop<T, V>(mut writer: V, mut stream: Receiver<T::Input>)
    where
        T: StreamSerializer,
        T::Input: Clone,
        V: DataCollector<T>
{
    while let Ok(data) = stream.recv().await {
        match writer.write(data) {
            Ok(_) => {},
            Err(e) => { error!("Failed to write data: {}", e); }
        }
    }
}