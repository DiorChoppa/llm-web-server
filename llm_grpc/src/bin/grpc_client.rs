use std::error::Error;

use llm_grpc::llm::{Empty, LlmQuery, llm_service_client::LlmServiceClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = LlmServiceClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(Empty {});
    let reponse = client.models(request).await?;
    for model in reponse.into_inner().result {
        println!("{}", model);
    }

    println!();

    let request = tonic::Request::new(LlmQuery {
        model: "llama3.2".to_string(),
        prompt: "Top 3 Rust lang disadvantages".to_string(),
    });
    let response = client.query(request).await?;
    for row in response.into_inner().result {
        println!("{}", row);
    }

    Ok(())
}
