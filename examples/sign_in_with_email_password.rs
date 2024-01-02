use clap::Parser;
use fars::Config;

#[derive(Parser)]
struct Credentials {
    #[arg(short, long)]
    email: String,
    #[arg(short, long)]
    password: String,
}

/// cargo run --example sign_in_with_email_password -- --email <email> --password <password>
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let credentials = Credentials::parse();

    // Read API key from the environment variable.
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Create a session by signing in with email and password.
    let _session = config
        .sign_in_with_email_password(
            credentials.email.clone(),
            credentials.password.clone(),
        )
        .await?;

    // Print the session.
    println!(
        "Succeeded to sign in with email/password: {}",
        credentials.email
    );

    Ok(())
}
