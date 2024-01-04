//! An example to sign in Twitter OAuth credential by session-based interface.
//!
//! ```shell
//! $ cargo run --example sign_in_twitter_oauth_credential -- --request_uri <request_uri> --access_token <access_token> --oauth_token_secret <oauth_token_secret>
//! ```

use clap::Parser;
use fars::data::IdpPostBody;
use fars::Config;

#[derive(Parser)]
struct Credentials {
    #[arg(short, long)]
    request_uri: String,
    #[arg(short, long)]
    access_token: String,
    #[arg(short, long)]
    oauth_token_secret: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let credentials = Credentials::parse();

    // Read API key from the environment variable.
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in Twitter OAuth credential.
    let _session = config
        .sign_in_oauth_credential(
            credentials
                .request_uri
                .clone(),
            IdpPostBody::Twitter {
                access_token: credentials
                    .access_token
                    .clone(),
                oauth_token_secret: credentials
                    .oauth_token_secret
                    .clone(),
            },
        )
        .await?;

    println!("Succeeded to sign in Twitter OAuth credential");

    Ok(())
}