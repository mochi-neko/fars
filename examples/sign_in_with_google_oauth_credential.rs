//! An example to sign in with Google OAuth credential by session-based interface.
//!
//! ```shell
//! $ cargo run --example sign_in_with_google_oauth_credential -- --request-uri <request_uri> --id-token <id_token>
//! ```

use clap::Parser;
use fars::ApiKey;
use fars::Config;
use fars::IdpPostBody;
use fars::OAuthRequestUri;

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
    let api_key = ApiKey::new(std::env::var("FIREBASE_API_KEY")?);

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in Google OAuth credential.
    let session = config
        .sign_in_with_oauth_credential(
            OAuthRequestUri::new(arguments.request_uri.clone()),
            IdpPostBody::Google {
                id_token: arguments.id_token.clone(),
            },
        )
        .await?;

    println!(
        "Succeeded to sign in with Google OAuth credential: {:?}",
        session
    );

    Ok(())
}
