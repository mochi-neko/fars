//! An example to get user data by session-based interface.
//!
//! ```shell
//! $ cargo run --example get_user_data -- --email <email> --password <password>
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

    // Get user data.
    let (_new_session, user_data) = session
        .get_user_data()
        .await?;

    println!(
        "Succeeded to get user data: {:?}",
        user_data
    );

    Ok(())
}
