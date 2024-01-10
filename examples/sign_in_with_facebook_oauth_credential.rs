//! An example to sign in with Facebook OAuth credential by session-based interface.
//!
//! NOTE: This example has not been tested.
//!
//! ```shell
//! $ cargo run --example sign_in_with_facebook_oauth_credential -- --request-uri <request_uri> --access-token <access_token>
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

    // Get a session by signing in Facebook OAuth credential.
    let session = config
        .sign_in_with_oauth_credential(
            OAuthRequestUri::new(arguments.request_uri.clone()),
            IdpPostBody::Facebook {
                access_token: arguments.access_token.clone(),
            },
        )
        .await?;

    println!(
        "Succeeded to sign in with Facebook OAuth credential: {:?}",
        session
    );

    Ok(())
}
