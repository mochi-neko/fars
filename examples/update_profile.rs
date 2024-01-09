//! An example to update profile by session-based interface.
//!
//! ```shell
//! $ cargo run --example update_profile -- --email <email> --password <password> --display-name <display_name> --photo-url <photo_url>
//! ```

use clap::Parser;
use fars::ApiKey;
use fars::Config;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    email: String,
    #[arg(long)]
    password: String,
    #[arg(short, long)]
    display_name: String,
    #[arg(long)]
    photo_url: String,
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
            arguments.email.clone(),
            arguments.password.clone(),
        )
        .await?;

    // Update profile.
    let session = session
        .update_profile(
            Some(arguments.display_name.clone()),
            Some(arguments.photo_url.clone()),
        )
        .await?;

    println!(
        "Succeeded to change email: {:?}",
        session
    );

    Ok(())
}
