//! An example to sign up with email and password by session-based interface.
//!
//! ```shell
//! $ cargo run --example sign_up_with_email_password -- --email <email> --password <password>
//! ```

use clap::Parser;
use fars::Config;

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
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing up with email and password.
    let session = config
        .sign_up_with_email_password(
            arguments.email.clone(),
            arguments.password.clone(),
        )
        .await?;

    println!(
        "Succeeded to sign up with email/password: {:?}",
        session
    );

    Ok(())
}