//! An example to fetch ID providers for specified email by session-based interface.
//!
//! ```shell
//! $ cargo run --example fetch_providers_for_email -- --email <email>
//! ```

use clap::Parser;
use fars::Config;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    email: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let arguments = Arguments::parse();

    // Read API key from the environment variable.
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Fetch ID providers for specified email.
    let providers = config
        .fetch_providers_for_email(
            arguments.email.clone(),
            "http://localhost".to_string(),
        )
        .await?;

    println!(
        "Succeeded to fetch ID providers for email: {:?}",
        providers
    );

    Ok(())
}
