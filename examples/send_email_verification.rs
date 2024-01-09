//! An example to send a verification email by session-based interface.
//!
//! ```shell
//! $ cargo run --example send_email_verification -- --email <email> --password <password>
//! ```

use clap::Parser;
use fars::{ApiKey, Config};

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
    let api_key = ApiKey::new(std::env::var("FIREBASE_API_KEY")?);

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in with email and password.
    let session = config
        .sign_in_with_email_password(
            arguments.email.clone(),
            arguments.password.clone(),
        )
        .await?;

    // Send a verification email.
    let session = session
        .send_email_verification(None)
        .await?;

    println!(
        "Succeeded to send a verification email: {:?}",
        session
    );

    Ok(())
}
