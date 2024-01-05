//! An example to sign in Google OAuth credential by session-based interface.
//!
//! ```shell
//! $ cargo run --example sign_in_google_oauth_credential -- --request_uri <request_uri> --id_token <id_token>
//! ```

use clap::Parser;
use fars::data::IdpPostBody;
use fars::Config;

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
    let credentials = Arguments::parse();

    // Read API key from the environment variable.
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in Google OAuth credential.
    let session = config
        .sign_in_oauth_credential(
            credentials
                .request_uri
                .clone(),
            IdpPostBody::Google {
                id_token: credentials.id_token.clone(),
            },
        )
        .await?;

    println!(
        "Succeeded to sign in with Google OAuth credential: {:?}",
        session
    );

    Ok(())
}
