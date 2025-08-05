use llm::llm_service_server::LlmService;
use llm::{Empty, LlmQuery, Strings};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::llm::llm_service_server::LlmServiceServer;

pub mod llm {
    tonic::include_proto!("llm");
}

#[derive(Debug, Default)]
pub struct LlmServiceImpl {}

#[tonic::async_trait]
impl LlmService for LlmServiceImpl {
    async fn models(&self, _request: Request<Empty>) -> Result<Response<Strings>, Status> {
        let result = match llm_calls::models().await {
            Ok(result) => result,
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        Ok(Response::new(Strings { result }))
    }

    async fn query(&self, request: Request<LlmQuery>) -> Result<Response<Strings>, Status> {
        let request = request.get_ref();
        let result = match llm_calls::query(&request.model, &request.prompt).await {
            Ok(result) => result,
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        Ok(Response::new(Strings { result }))
    }
}

pub async fn run_llm_grpc() -> Result<(), Box<dyn std::error::Error>> {
    Server::builder()
        .add_service(LlmServiceServer::new(LlmServiceImpl::default()))
        .serve("127.0.0.1:50051".parse()?)
        .await?;

    Ok(())
}
