//! An example to link with Google OAuth credential by session-based interface.
//! ```shell
//! $ cargo run --example link_with_google -- --request-uri <request_uri> --id-token <id_token>
//! ```

use clap::Parser;
use fars::ApiKey;
use fars::Config;
use fars::IdpPostBody;

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

    // Get a session by signing in anonymously.
    let session = config
        .sign_in_anonymously()
        .await?;

    // Link with Google OAuth credential.
    let session = session
        .link_with_oauth_credential(
            arguments.request_uri.clone(),
            IdpPostBody::Google {
                id_token: arguments.id_token.clone(),
            },
        )
        .await?;

    println!(
        "Succeeded to link Google OAuth credential: {:?}",
        session
    );

    // Delete the anonymous account.
    session
        .delete_account()
        .await?;

    Ok(())
}
