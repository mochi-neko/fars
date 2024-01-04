//! An example to sign in with email and password by raw API interface.
//!
//! ```shell
//! $ cargo run --example raw_sign_in_with_email_password -- --email <email> --password <password>
//! ```

use clap::Parser;
use fars::api;

#[derive(Parser)]
struct Credentials {
    #[arg(short, long)]
    email: String,
    #[arg(short, long)]
    password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let credentials = Credentials::parse();

    // Read API key from the environment variable.
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a reqwest client.
    let client = reqwest::Client::new();

    // Create a request payload
    let request_payload = api::SignInWithEmailPasswordRequestBodyPayload::new(
        credentials.email.clone(),
        credentials.password.clone(),
    );

    // Get a response by signing in with email and password.
    let response_payload =
        api::sign_in_with_email_password(&client, &api_key, request_payload)
            .await?;

    println!(
        "Succeeded to sign in with email/password: {}",
        response_payload.email
    );

    Ok(())
}
