//! An example to unlink email and password by session-based interface.
//!
//! ```shell
//! $ cargo run --example unlink_password -- --email <email> --password <password>
//! ```

use clap::Parser;
use fars::{data::ProviderId, Config};

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
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in with email and password.
    let session = config
        .sign_in_with_email_password(
            arguments.email.clone(),
            arguments.password.clone(),
        )
        .await?;

    // Unlink email and password.
    let session = session
        .unlink_provider(
            [ProviderId::Password]
                .iter()
                .cloned()
                .collect(),
        )
        .await?;

    println!(
        "Succeeded to unlink email and password: {:?}",
        session
    );

    Ok(())
}
