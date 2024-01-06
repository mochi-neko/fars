//! An example to unlink Google OAuth credential by session-based interface.
//!
//! ```shell
//! $ cargo run --example unlink_google -- --request-uri <request_uri> --id-token <id_token>
//! ```

use clap::Parser;
use fars::{
    data::{IdpPostBody, ProviderId},
    Config,
};

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    request_uri: String,
    #[arg(short, long)]
    id_token: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let arguments = Arguments::parse();

    // Read API key from the environment variable.
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in anonymously.
    let session = config
        .sign_in_anonymously()
        .await?;

    let session = session
        .link_with_oauth_credential(
            arguments.request_uri.clone(),
            IdpPostBody::Google {
                id_token: arguments.id_token.clone(),
            },
        )
        .await?;

    // Unlink Google OAuth credential.
    let session = session
        .unlink_provider(
            [ProviderId::Google]
                .iter()
                .cloned()
                .collect(),
        )
        .await?;

    println!(
        "Succeeded to unlink Google OAuth credential: {:?}",
        session
    );

    // Delete the anonymous account.
    session
        .delete_account()
        .await?;

    Ok(())
}
