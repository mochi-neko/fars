//! An example code to verify an ID token of the Firebase Auth for signing in user.
//!
//! ```shell
//! $ cargo run --example verify_id_token --features verify -- --email <email> --password <password>
//! ```

#![cfg(feature = "verify")]

use clap::Parser;

use fars::verification::VerificationConfig;
use fars::ApiKey;
use fars::Config;
use fars::Email;
use fars::Password;
use fars::ProjectId;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    email: String,
    #[arg(short, long)]
    password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let arguments = Arguments::parse();

    // Read API key from the environment variable.
    let api_key = ApiKey::from_env()?;

    // Read project ID from the environment variable.
    let project_id = ProjectId::from_env()?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in with email and password.
    let session = config
        .sign_in_with_email_password(
            Email::new(arguments.email.clone()),
            Password::new(arguments.password.clone()),
        )
        .await?;

    // Create a verification config.
    let config = VerificationConfig::new(project_id);

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
