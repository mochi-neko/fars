//! An example to sign in with refresh token by session-based interface.
//!
//! ```shell
//! $ cargo run --example sign_in_with_refresh_token -- --refresh-token <refresh_token>
//! ```

use clap::Parser;
use fars::ApiKey;
use fars::Config;
use fars::RefreshToken;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    refresh_token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let arguments = Arguments::parse();

    // Read API key from the environment variable.
    let api_key = ApiKey::new(std::env::var("FIREBASE_API_KEY")?);

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by exchanging refresh token.
    let session = config
        .exchange_refresh_token(RefreshToken::new(
            arguments
                .refresh_token
                .clone(),
        ))
        .await?;

    println!(
        "Succeeded to sign in by refresh token: {:?}",
        session
    );

    Ok(())
}
