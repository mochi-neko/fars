//! An example to sign in anonymously by session-based interface.
//!
//! ```shell
//! $ cargo run --example sign_in_anonymously
//! ```

use fars::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read API key from the environment variable.
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Get a session by signing in anonymously.
    let _session = config
        .sign_in_anonymously()
        .await?;

    println!("Succeeded to sign in anonymously");

    Ok(())
}
