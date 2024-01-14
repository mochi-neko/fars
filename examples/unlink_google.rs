//! An example to unlink Google OAuth credential by session-based interface.
//!
//! ```shell
//! $ cargo run --example unlink_google -- --request-uri <request_uri> --access-token <access_token>
//! ```

use std::collections::HashMap;
use std::collections::HashSet;

use clap::Parser;
use fars::ApiKey;
use fars::Config;
use fars::IdpPostBody;
use fars::OAuthRequestUri;
use fars::ProviderId;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    request_uri: String,
    #[arg(short, long)]
    access_token: String,
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

    let session = session
        .link_with_oauth_credential(
            OAuthRequestUri::new(arguments.request_uri.clone()),
            IdpPostBody::new(
                ProviderId::Google,
                HashMap::from([(
                    "access_token",
                    arguments.access_token.clone(),
                )]),
            )?,
        )
        .await?;

    // Unlink Google OAuth credential.
    let session = session
        .unlink_provider(HashSet::from([ProviderId::Google]))
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
