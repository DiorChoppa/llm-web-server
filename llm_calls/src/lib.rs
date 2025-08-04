use std::error::Error;
use tokio::process::Command;

pub async fn query(model: &str, prompt: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let output = Command::new("ollama")
        .arg("run")
        .arg(model)
        .arg(prompt)
        .output()
        .await?;

    if !output.status.success() {
        return Err("ollama run failed".into());
    }

    let result = String::from_utf8(output.stdout)?
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(result)
}

pub async fn models() -> Result<Vec<String>, Box<dyn Error>> {
    let output = Command::new("ollama").arg("list").output().await?;

    if !output.status.success() {
        return Err("ollama list failed".into());
    }

    let result = String::from_utf8(output.stdout)?
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn models_healthy() {
        let models = super::models().await.unwrap();
        dbg!(&models);
        assert!(!models.is_empty());
    }

    #[tokio::test]
    async fn query_healthy() {
        let reponse = super::query("llama3.2", "hello!").await.unwrap();

        dbg!(&reponse);
        assert!(!reponse.is_empty());
    }
}
