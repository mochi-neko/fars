//! An example to sign in with Facebook OAuth credential by session-based interface
//! on the Device Code grant type of the OAuth 2.0.
//!
//! ```shell
//! $ cargo run --example sign_in_with_facebook_oauth_credential_on_device_code --features oauth
//! ```

#![cfg(feature = "oauth")]

use qrcode::render::unicode;
use std::collections::HashSet;

use fars::oauth::FacebookDeviceCodeClient;
use fars::oauth::OAuthScope;
use fars::ApiKey;
use fars::Config;
use fars::OAuthRequestUri;
use fars::ProviderId;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get Client ID from the environment variables.
    let app_id = std::env::var("FACEBOOK_CLIENT_ID")?;
    let client_token = std::env::var("FACEBOOK_CLIENT_TOKEN")?;

    // Create an OAuth client.
    let oauth_client = FacebookDeviceCodeClient::new(app_id, client_token)?;

    // Request authorization.
    let session = oauth_client
        .request_authorization(HashSet::from([
            OAuthScope::open_id(),
            OAuthScope::open_id_email(),
            OAuthScope::open_id_profile(),
        ]))
        .await?;

    let verification_uri = session.verification_uri();
    let user_code = session.user_code();

    // Encode verification URI to QR code.
    let qr_code = qrcode::QrCode::new(verification_uri)?;

    // Render QR code as string on the terminal.
    let qr_code_string = qr_code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();

    // Display the verification URI and user code to the terminal.
    println!("Verification URI:\n{}", qr_code_string);
    println!("User code: {}", user_code);

    // Poll to token endpoint to exchange a device code into an access token.
    let token = session
        .poll_exchange_token(tokio::time::sleep, None)
        .await?;

    // Sign in with Facebook OAuth credential.
    let config = Config::new(ApiKey::from_env()?);
    let session = config
        .sign_in_with_oauth_credential(
            OAuthRequestUri::new("http://localhost:8080"),
            token.create_idp_post_body(ProviderId::Facebook)?,
        )
        .await?;

    println!(
        "Succeeded to sign in with Facebook OAuth credential: {:?}",
        session
    );

    Ok(())
}
