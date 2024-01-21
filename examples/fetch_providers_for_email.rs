//! An example to fetch ID providers for specified email by session-based interface.
//!
//! ```shell
//! $ cargo run --example fetch_providers_for_email -- --email <email>
//! ```

use clap::Parser;
use fars::ApiKey;
use fars::Config;
use fars::Email;
use fars::OAuthContinueUri;

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
    let api_key = ApiKey::from_env()?;

    // Create a config.
    let config = Config::new(api_key);

    // Fetch ID providers for specified email.
    let providers = config
        .fetch_providers_for_email(
            Email::new(arguments.email.clone()),
            OAuthContinueUri::new("http://localhost"),
        )
        .await?;

    // NOTE:
    // Because email enumeration protection is enabled by default,
    // the response may be `None`.
    // See also the issue: https://github.com/firebase/firebase-ios-sdk/issues/11810
    println!(
        "Succeeded to fetch ID providers for email: {:?}",
        providers
    );

    Ok(())
}
