//! An example to sign in by refresh token by session-based interface.
//!
//! ```shell
//! $ cargo run --example sign_in_by_refresh_token -- --refresh_token <refresh_token>
//! ```

use clap::Parser;
use fars::Config;

#[derive(Parser)]
struct Credentials {
    #[arg(short, long)]
    refresh_token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let credentials = Credentials::parse();

    // Read API key from the environment variable.
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by exchanging refresh token.
    let _session = config
        .exchange_refresh_token(
            credentials
                .refresh_token
                .clone(),
        )
        .await?;

    println!(
        "Succeeded to sign in by refresh token: {}",
        credentials.refresh_token
    );

    Ok(())
}
