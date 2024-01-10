//! An example to change password by session-based interface.
//!
//! ```shell
//! $ cargo run --example change_password -- --email <email> --password <password> --new-password <new-password>
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
    new_password: String,
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

    // Change password.
    let session = session
        .change_password(arguments.new_password.clone())
        .await?;

    println!(
        "Succeeded to change password: {:?}",
        session
    );

    Ok(())
}
