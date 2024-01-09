//! An example to refresh ID token by session-based interface.
//!
//! ```shell
//! $ cargo run --example refresh_token -- --email <email> --password <password>
//! ```

use clap::Parser;
use fars::ApiKey;
use fars::Config;

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

    // Refresh ID token.
    let session = session
        .refresh_token()
        .await?;

    println!(
        "Succeeded to refresh ID token: {:?}",
        session
    );

    Ok(())
}
