//! An example to unlink email and password by session-based interface.
//!
//! ```shell
//! $ cargo run --example unlink_password -- --email <email> --password <password>
//! ```

use std::collections::HashSet;

use clap::Parser;
use fars::ApiKey;
use fars::Config;
use fars::Email;
use fars::Password;
use fars::ProviderId;

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

    // Get a session by signing in anonymously.
    let session = config
        .sign_in_anonymously()
        .await?;

    // Link email and password.
    let session = session
        .link_with_email_password(
            Email::new(arguments.email.clone()),
            Password::new(arguments.password.clone()),
        )
        .await?;

    // Unlink email and password.
    let session = session
        .unlink_provider(HashSet::from([ProviderId::Password]))
        .await?;

    println!(
        "Succeeded to unlink email and password: {:?}",
        session
    );

    // Delete the anonymous account.
    session
        .delete_account()
        .await?;

    Ok(())
}
