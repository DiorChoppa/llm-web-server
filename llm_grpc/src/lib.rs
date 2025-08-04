use crate::llm::{Empty, LlmQuery, Strings};
use llm::llm_service_server::LlmService;
use tonic::{Request, Response, Status};

pub mod llm {
    tonic::include_proto!("llm");
}

#[derive(Debug, Default)]
pub struct LlmServiceImpl {}

#[tonic::async_trait]
impl LlmService for LlmServiceImpl {
    async fn models(&self, _request: Request<Empty>) -> Result<Response<llm::Strings>, Status> {
        todo!()
    }

    async fn query(&self, request: Request<LlmQuery>) -> Result<Response<Strings>, Status> {
        todo!()
    }
}
