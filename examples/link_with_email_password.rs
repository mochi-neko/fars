//! An example to link with email and password by session-based interface.
//!
//! ```shell
//! $ cargo run --example link_with_email_password -- --email <email> --password <password>
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let arguments = Arguments::parse();

    // Read API key from the environment variable.
    let api_key = ApiKey::from_env()?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in anonymously.
    let session = config
        .sign_in_anonymously()
        .await?;

    // Link email and password.
    let session = session
        .link_with_email_password(
            Email::new(arguments.email.clone()),
            Password::new(arguments.password.clone()),
        )
        .await?;

    println!(
        "Succeeded to link with email/password: {:?}",
        session
    );

    // Delete the anonymous account.
    session
        .delete_account()
        .await?;

    Ok(())
}
