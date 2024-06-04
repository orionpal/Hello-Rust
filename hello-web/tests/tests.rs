use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_hello_world() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let res = client.get("http://localhost:8080/")
        .send()
        .await?
        .text()
        .await?;

    assert_eq!(res, "Hello, World!");
    Ok(())
}

#[tokio::test]
async fn test_hello_user() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let res = client.get("http://localhost:8080/hello/Orion")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    assert_eq!(res, json!({"message": "Hello, Orion!"}));
    Ok(())
}

#[tokio::test]
async fn test_greet() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let res = client.post("http://localhost:8080/greet")
        .json(&json!({"name": "Orion"}))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    assert_eq!(res, json!({"message": "Hello, Orion!"}));
    Ok(())
}

#[tokio::test]
async fn test_greet_empty_name() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let res = client.post("http://localhost:8080/greet")
        .json(&json!({"name": ""}))
        .send()
        .await?;

    assert_eq!(res.status(), reqwest::StatusCode::BAD_REQUEST);

    let res_body = res.json::<serde_json::Value>()
        .await?;
    assert_eq!(res_body, json!({"error": "Name cannot be empty"}));
    Ok(())
}
