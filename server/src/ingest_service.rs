use crate::process::process_batch;
use crate::state::Shared;
use ingest_proto::ecg::ingest::v1::{
    ingestor_server::{Ingestor, IngestorServer},
    Ack, SampleBatch,
};
use tonic::{Request, Response, Status};
use tracing::{info, warn};

pub struct IngestService {
    shared: Shared,
}

impl IngestService {
    pub fn new(shared: Shared) -> Self { Self { shared } }
}

#[tonic::async_trait]
impl Ingestor for IngestService {
    // Unary: one batch per request
    async fn ingest_once(&self, request: Request<SampleBatch>) -> Result<Response<Ack>, Status> {
        let batch = request.into_inner();
        let sid = batch.session_id.clone();
        let (received, dropped, last_seq, warning) = process_batch(&self.shared, batch).await;

        info!(session = %sid, received, dropped, last_seq, warning = %warning, "IngestOnce");
        let reply = Ack { received, dropped, last_seq, warning };
        Ok(Response::new(reply))
    }

    // Client-streaming: many batches on one connection
    async fn ingest(
        &self,
        request: Request<tonic::Streaming<SampleBatch>>,
    ) -> Result<Response<Ack>, Status> {
        let mut stream = request.into_inner();

        let mut total_received: u64 = 0;
        let mut total_dropped: u64 = 0;
        let mut last_seq: u64 = 0;
        let mut last_sid: String = String::new();
        let mut final_warning = String::new();

        while let Some(batch) = stream.message().await.map_err(map_status)? {
            last_sid = batch.session_id.clone();
            let (rec, dropc, last, warn) = process_batch(&self.shared, batch).await;
            total_received += rec;
            total_dropped += dropc;
            if last > last_seq { last_seq = last; }
            if !warn.is_empty() { final_warning = warn; }
        }

        info!(session = %last_sid, received = total_received, dropped = total_dropped, last_seq, warning = %final_warning, "Ingest (stream end)");
        Ok(Response::new(Ack {
            received: total_received,
            dropped: total_dropped,
            last_seq,
            warning: final_warning,
        }))
    }
}

fn map_status<E: std::fmt::Display>(e: E) -> Status {
    warn!("stream read error: {e}");
    Status::internal(format!("stream error: {e}"))
}

// Re-export server type for main.rs (handy if you prefer importing here)
// pub type IngestorGrpcServer = IngestorServer<IngestService>;
