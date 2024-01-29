//! An example to sign in with Google OAuth credential by session-based interface
//! on the Device Code grant type of the OAuth 2.0.
//!
//! ```shell
//! $ cargo run --example sign_in_with_google_oauth_credential_on_device_code --features oauth
//! ```

#![cfg(feature = "oauth")]

use std::collections::HashSet;

use fars::oauth::ClientId;
use fars::oauth::ClientSecret;
use fars::oauth::GoogleDeviceCodeClient;
use fars::oauth::OAuthScope;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get Client ID and Client Secret from the environment variables.
    let client_id = ClientId::from_env("GOOGLE_CLIENT_ID")?;
    let client_secret = ClientSecret::from_env("GOOGLE_CLIENT_SECRET")?;

    // Create an OAuth client.
    let oauth_client = GoogleDeviceCodeClient::new(client_id, client_secret)?;

    // Request authorization.
    let session = oauth_client
        .request_authorization(HashSet::from([
            OAuthScope::open_id(),
            OAuthScope::open_id_email(),
            OAuthScope::open_id_profile(),
        ]))
        .await?;

    // Display the verification URI and user code to the user.
    println!(
        "Verification URI: {}",
        session.verification_uri
    );

    Ok(())
}
