use std::{convert::Infallible, sync::Arc, time::Duration};

use aws_config::Region;
use aws_sdk_s3::config::{Credentials, SharedCredentialsProvider};
use futures::{Stream, StreamExt};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber;
use api::api_route;
use axum::{extract::{DefaultBodyLimit, Path}, http::{header, Method}, response::{sse::{Event, KeepAlive}, Sse}, routing::get, Extension, Router};
use sea_orm::Database;
use tracing::{info, error};
use services::{db_connection::{ReportBroadcaster, DB}, dotenv::Dotenv};
use tokio_stream::wrappers::BroadcastStream;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();
    
    let env = Dotenv::new();

    if env.is_some() {
        info!("env value parsed");
    } else {
        error!("failed to parsed env");
        panic!();
    }

    let env = env.unwrap();

    let db = Database::connect(env.read_value("DATABASE_URL")).await;

    if let Ok(_) = db {
        info!("DB Connection Established");
    } else {
        error!("DB Connection Not Established");
        panic!();
    }

    let redis_client = redis::Client::open(env.read_value("REDIS_URL"));

    if let Ok(_) = redis_client {
        info!("Redis Connection Established");
    } else {
        error!("Redis Connection Not Established");
        panic!();
    }

    let credentials = Credentials::new(
        env.read_value("garage_key_id"),
        env.read_value("garage_key_secret"),
        None, // session_token
        None, // expires_at
        "garage-s3-credentials" // provider_name
    );

    let s3_config = aws_config::SdkConfig::builder()
        .endpoint_url(env.read_value("garage_api_url"))
        .region(Region::new("garage"))
        .credentials_provider(SharedCredentialsProvider::new(credentials))
        .build();
    let s3_client = aws_sdk_s3::Client::new(&s3_config);

    info!("S3 Connection Established");

    let con: redis::aio::MultiplexedConnection = redis_client.unwrap().get_multiplexed_async_connection().await.unwrap();

    let sse_broadcaster = ReportBroadcaster::new();

    let shared_state = Arc::new(DB {
        db_connection: db.unwrap(),
        redis_connection: con,
        s3_connection: s3_client,
        sse_connection: sse_broadcaster,
    });

    let app = Router::new()
        .nest("/api", api_route())
        .route("/sse-connection/{event_id}", get(sse_handler)).layer(
            ServiceBuilder::new()
                .layer(Extension(shared_state))
                .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
                .layer(CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST])
                    .allow_origin(Any)
                    .allow_headers([
                        header::CONTENT_TYPE
                    ])
                )
        )
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}


async fn sse_handler(
    Extension(db): Extension<Arc<DB>>,
    Path(event_id): Path<String>
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {

    let rx = db.sse_connection.sender.subscribe();

    let stream = BroadcastStream::new(rx)
        .filter_map(move |result| {
            let event_data = event_id.to_owned();

            async move {
                match result {
                    Ok(event) => {
                        let sse_event = Event::default()
                            .data(serde_json::to_string(&event.data).unwrap())
                            .event(event.event_type.clone())
                            .id(&event_data);
                        Some(Ok(sse_event))
                    }
                    Err(_) => None,
                }
            }
        });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive")
    )
}