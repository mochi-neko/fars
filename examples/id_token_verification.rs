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
        // Create a verification config.
        let config =
            fars::verification::VerificationConfig::new(project_id.clone());

        // Verify the ID token.
        let claims = config
            .verify_id_token(&session.id_token)
            .await?;

        println!(
            "Token ID verification succeeded: {:?}",
            claims
        );

        return Ok(());
    }

    Err(anyhow::anyhow!(
        "Feature \"verify\" is not enabled.",
    ))
}
