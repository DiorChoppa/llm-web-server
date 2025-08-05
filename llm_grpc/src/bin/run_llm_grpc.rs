use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    llm_grpc::run_llm_grpc().await
}
