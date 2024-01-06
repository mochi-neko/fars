//! An example to sign in Facebook OAuth credential by session-based interface.
//!
//! NOTE: This example has not been tested.
//!
//! ```shell
//! $ cargo run --example sign_in_facebook_oauth_credential -- --request-uri <request_uri> --access-token <access_token>
//! ```

use clap::Parser;
use fars::data::IdpPostBody;
use fars::Config;

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
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in Facebook OAuth credential.
    let session = config
        .sign_in_oauth_credential(
            arguments.request_uri.clone(),
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
