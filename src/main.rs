// reqwest is the HTML library for Rust
use reqwest::{Client, Error as ReqwestError};
// serde is for serializing and deserializing JSON data.
use serde::{Deserialize, Serialize};

// derive(Debug) automatically implements the Debug trait for a struct.
// derive(Serialize, Deserialize) is an attribute provided by the serde crate for serializing and deserializing in Rust (JSON).
#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    access_token: String,
    instance_url: String
}

// client_id: &str, client_secret: &str, username: &str, password: &str immutable references to string slices.
// -> Result<AuthResponse, ReqwestError> return type of the function.
async fn authenticate(client_id: &str, client_secret: &str, username: &str, password: &str) -> Result<AuthResponse, ReqwestError> {
    let client = Client::new();
    let params = [
        ("grant_type", "password"),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("username", username),
        ("password", password),
    ];

    let auth_url = "https://na18-dev-ed.develop.my.salesforce.com/services/oauth2/token";

    let res = client
        .post(auth_url)
        .form(&params)
        .send()
        .await?;

    // match is a control flow construct in Rust that allows for you to handle different possible values of a type, especially enums. Similar to Switch-Case statements but more powerful.
    match res.error_for_status_ref() {
        // Ok and Err are the two variants of the result enum. Ok indicates success and contains the successful values and Err indicates an error and contains the error information.
        Ok(_) => res.json::<AuthResponse>().await,
        Err(error) => {
            // Log the error response text for debugging
            let err_text = res.text().await.unwrap_or_else(|_| "Failed to read error response".to_string());
            eprintln!("Failed to authenticate. Response: {}", err_text);
            Err(error)
        }
    }
}

async fn query_salesforce(instance_url: &str, access_token: &str, query: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let request_url = format!("{}/services/data/v58.0/query/?q={}", instance_url, query);
    let res = client
        .get(&request_url)
        .bearer_auth(access_token)
        .send()
        .await?
        .text()
        .await?;

    println!("Response: {}", res);
    Ok(())
}

#[tokio::main] // Asynchronous runtime for Rust.
async fn main() {
    let client_id = "your_client_id";
    let client_secret = "your_client_secret";
    let username = "your_username";
    let password = "your_password_with_security_token";

    match authenticate(client_id, client_secret, username, password).await {
        Ok(auth_response) => {
            let query = "SELECT Id, Name FROM Account";
            if let Err(e) = query_salesforce(&auth_response.instance_url, &auth_response.access_token, query).await {
                eprintln!("Error querying Salesforce: {}", e);
            }
        }
        Err(e) => eprintln!("Error authenticating: {}", e),
    }
}