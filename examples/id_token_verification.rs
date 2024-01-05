//! An example code to verify an ID token of the Firebase Auth for signing in user.
//!
//! ```shell
//! $ cargo run --example id_token_verification --features verify -- --email <email> --password <password>
//! ```

use clap::Parser;
use fars::Config;

#[derive(Parser)]
struct Credentials {
    #[arg(short, long)]
    email: String,
    #[arg(short, long)]
    password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let credentials = Credentials::parse();

    // Read API key from the environment variable.
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Read project ID from the environment variable.
    #[allow(unused_variables)]
    let project_id = std::env::var("FIREBASE_PROJECT_ID")?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in with email and password.
    #[allow(unused_variables)]
    let session = config
        .sign_in_with_email_password(
            credentials.email.clone(),
            credentials.password.clone(),
        )
        .await?;

    #[cfg(feature = "verify")]
    {
        return verify(&session.id_token, &project_id).await;
    }

    Err(anyhow::anyhow!(
        "Feature \"verify\" is not enabled.",
    ))
}

#[cfg(feature = "verify")]
async fn verify(
    id_token: &String,
    project_id: &String,
) -> anyhow::Result<()> {
    // Verify the ID token.
    let claims = fars::verification::verify_id_token(
        &reqwest::Client::new(),
        id_token,
        &project_id,
    )
    .await?;

    println!(
        "Token ID verification succeeded: {:?}",
        claims
    );

    Ok(())
}
