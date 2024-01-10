//! An example to change email by session-based interface.
//!
//! ```shell
//! $ cargo run --example change_email -- --email <email> --password <password> --new-email <new-email>
//! ```

use clap::Parser;
use fars::ApiKey;
use fars::Config;
use fars::Email;
use fars::Password;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    email: String,
    #[arg(short, long)]
    password: String,
    #[arg(short, long)]
    new_email: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let arguments = Arguments::parse();

    // Read API key from the environment variable.
    let api_key = ApiKey::new(std::env::var("FIREBASE_API_KEY")?);

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in with email and password.
    let session = config
        .sign_in_with_email_password(
            Email::new(arguments.email.clone()),
            Password::new(arguments.password.clone()),
        )
        .await?;

    // Change email.
    let session = session
        .change_email(
            Email::new(arguments.new_email.clone()),
            None,
        )
        .await?;

    println!(
        "Succeeded to change email: {:?}",
        session
    );

    Ok(())
}
