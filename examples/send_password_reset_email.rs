//! An example to send a password reset email by session-based interface.
//!
//! ```shell
//! $ cargo run --example send_password_reset_email -- --email <email>
//! ```

use clap::Parser;
use fars::ApiKey;
use fars::Config;
use fars::Email;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    email: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let arguments = Arguments::parse();

    // Read API key from the environment variable.
    let api_key = ApiKey::from_env()?;

    // Create a config.
    let config = Config::new(api_key);

    // Send a password reset email.
    config
        .send_reset_password_email(
            Email::new(arguments.email.clone()),
            None,
        )
        .await?;

    println!(
        "Succeeded to send a password reset email to {}",
        arguments.email
    );

    Ok(())
}
