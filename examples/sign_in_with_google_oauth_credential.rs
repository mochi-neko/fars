//! An example to sign in with Google OAuth credential by session-based interface.
//!
//! ```shell
//! $ cargo run --example sign_in_with_google_oauth_credential --features oauth
//! ```

#![cfg(feature = "oauth")]

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::{routing::get, Router};
use serde::Deserialize;
use tokio::sync::{mpsc, Mutex};

use fars::oauth::OAuthAuthorizationCode;
use fars::oauth::OAuthAuthorizationState;
use fars::oauth::OAuthClientId;
use fars::oauth::OAuthClientSecret;
use fars::oauth::OAuthGoogleClient;
use fars::oauth::OAuthRedirectUrl;
use fars::oauth::OAuthScope;
use fars::oauth::OAuthSession;
use fars::ApiKey;
use fars::Config;
use fars::IdpPostBody;
use fars::OAuthRequestUri;
use fars::ProviderId;

#[derive(Clone)]
struct ServerState {
    config: Arc<Mutex<Config>>,
    oauth_session: Arc<Mutex<OAuthSession>>,
    tx: mpsc::Sender<()>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct QueryParameters {
    code: Option<String>,
    cope: Option<String>,
    authuser: Option<usize>,
    prompt: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

async fn handle_redirect(
    state: State<ServerState>,
    Query(params): Query<QueryParameters>,
) -> String {
    // Check query parameters.
    if let Some(error) = params.error {
        eprintln!("Error: {}", error);
        return "".to_string();
    }

    let auth_code;
    if let Some(code) = params.code {
        auth_code = code;
    } else {
        eprintln!("Error: No authorization code.");
        return "".to_string();
    }

    let auth_state;
    if let Some(param_state) = params.state {
        auth_state = param_state;
    } else {
        eprintln!("Error: No state.");
        return "".to_string();
    }

    // Continue to sign in process.
    match continue_sign_in(state, auth_code, auth_state).await {
        | Ok(_) => {
            "Succeeded to sign in with Google OAuth credential.".to_string()
        },
        | Err(e) => {
            eprintln!("Error: {:?}", e);
            "".to_string()
        },
    }
}

async fn continue_sign_in(
    state: State<ServerState>,
    auth_code: String,
    auth_state: String,
) -> anyhow::Result<()> {
    let oauth_session = state
        .oauth_session
        .lock()
        .await;
    let sender = state.tx.clone();

    // Exchange authorization code into OAuth token.
    let token = oauth_session
        .exchange_code_into_token(
            OAuthAuthorizationCode::new(auth_code),
            OAuthAuthorizationState::new(auth_state),
        )
        .await
        .map_err(|e| {
            // Stop server
            tokio::spawn(async move {
                sender.send(()).await.unwrap();
            });
            anyhow::anyhow!("{:?}", e)
        })?;

    let config = state.config.lock().await;
    let sender = state.tx.clone();

    // Get a session by signing in Google OAuth credential.
    let session = config
        .sign_in_with_oauth_credential(
            OAuthRequestUri::new("http://localhost:8080"),
            IdpPostBody::new(
                ProviderId::Google,
                HashMap::from([(
                    "access_token",
                    token
                        .access_token
                        .inner()
                        .to_owned(),
                )]),
            )?,
        )
        .await
        .map_err(|e| {
            // Stop server
            tokio::spawn(async move {
                sender.send(()).await.unwrap();
            });
            anyhow::anyhow!("{:?}", e)
        })?;

    println!(
        "Succeeded to sign in with Google OAuth credential: {:?}",
        session
    );

    // Stop server
    let sender = state.tx.clone();
    sender.send(()).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get secrets from the environment variables.
    let google_client_id =
        OAuthClientId::new(std::env::var("GOOGLE_CLIENT_ID")?);
    let google_client_secret =
        OAuthClientSecret::new(std::env::var("GOOGLE_CLIENT_SECRET")?);

    // Create an OAuth client.
    let oauth_client = OAuthGoogleClient::new(
        google_client_id,
        google_client_secret,
        OAuthRedirectUrl::new("http://localhost:8080/auth/google-callback")?,
    )?;

    // Generate an OAuth session with authorization URL.
    let session = oauth_client.generate_authorization_url(HashSet::from([
        OAuthScope::new("https://www.googleapis.com/auth/userinfo.email"),
        OAuthScope::new("https://www.googleapis.com/auth/userinfo.profile"),
        OAuthScope::new("openid"),
    ]));

    // Open the authorization URL in the default browser.
    webbrowser::open(session.url.inner())?;

    // Create a channel to receive a signal to stop the server.
    let (tx, mut rx) = mpsc::channel::<()>(1);

    // Create a server state.
    let server_state = ServerState {
        config: Arc::new(Mutex::new(Config::new(ApiKey::new(
            std::env::var("FIREBASE_API_KEY")?,
        )))),
        oauth_session: Arc::new(Mutex::new(session)),
        tx,
    };

    // Build application with redirection handler.
    let app = Router::new()
        .route(
            "/auth/google-callback",
            get(handle_redirect),
        )
        .with_state(server_state);

    // Run it with hyper on localhost:8080 to receive the authorization code by redirection.
    let listener = tokio::net::TcpListener::bind("localhost:8080").await?;

    // Wait for the server to stop or receive a signal to stop the server.
    tokio::select! {
        | _ = rx.recv() => {
            println!("Received a signal to stop the server.");
        },
        | _ = async { axum::serve(listener, app).await } => {
            println!("Server stopped.");
        },
    }

    Ok(())
}
