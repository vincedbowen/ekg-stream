mod config;
mod state;
mod process;
mod ingest_service;

use anyhow::Result;
use tonic::transport::Server;

use ingest_service::IngestService;
use ingest_proto::ecg::ingest::v1::ingestor_server::IngestorServer;

#[tokio::main]
async fn main() -> Result<()> {
    // config
    let cfg = config::Config::from_env();
    let addr = cfg.bind_addr.parse()?;

    // shared state + service
    let shared = state::Shared::default();
    let svc = IngestService::new(shared.clone());

    tracing::info!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(IngestorServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
