use minio::s3::Client;
use redis::aio::MultiplexedConnection;
use sea_orm::{DatabaseConnection};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportEvent {
    pub id: String,
    pub event_type: String,
    pub data: serde_json::Value,
    pub timestamp: i64,
}

#[derive(Clone)]
pub struct ReportBroadcaster {
    pub sender: broadcast::Sender<ReportEvent>
}

impl ReportBroadcaster {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn send_progress_update(&self, report_id: &str, progress: u32, message: &str) {
        let event = ReportEvent {
            id: report_id.to_string(),
            event_type: "report_progress".to_string(),
            data: serde_json::json!({
                "id": report_id,
                "progress": progress,
                "message": message,
                "status": "processing"
            }),
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        let _ = self.sender.send(event);
    }
}

pub struct DB {
    pub db_connection: DatabaseConnection,
    pub redis_connection: MultiplexedConnection,
    // pub s3_connection: Client,
    pub minio_connection: Client,
    pub sse_connection: ReportBroadcaster
}

