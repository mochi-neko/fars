//! An example to customize HTTP client for APIs.
//!
//! ```shell
//! $ cargo run --example customize_http_client --features custom_client -- --email <email> --password <password>
//! ```

#![cfg(feature = "custom_client")]

use clap::Parser;

use std::time::Duration;

use fars::ApiKey;
use fars::Client;
use fars::Config;
use fars::Email;
use fars::Password;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    email: String,
    #[arg(short, long)]
    password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let arguments = Arguments::parse();

    // Read API key from the environment variable.
    let api_key = ApiKey::from_env()?;

    // Create a custom reqwest client with timeout.
    let client = fars::reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(10))
        .build()?;

    // Customize HTTP client.
    let client = Client::custom(client);

    // Create a config.
    let config = Config::custom(api_key, client);

    // Get a session by signing in with email and password.
    let session = config
        .sign_in_with_email_password(
            Email::new(arguments.email.clone()),
            Password::new(arguments.password.clone()),
        )
        .await?;

    println!(
        "Succeeded to sign in with email/password: {:?}",
        session
    );

    Ok(())
}
