# fars

An unofficial Rust client for the [Firebase Auth REST API](https://firebase.google.com/docs/reference/rest/auth).

## Installation

Please install this library by CLI:

```shell
$ cargo add fars
```

or adding dependency to your `Cargo.toml`:

```toml
[dependencies]
fars = "0.2.0"
```

## Features
All features in this crate are as follows:

- default
    - [Session-based interfaces](#api-usages)
    - [Raw API interfaces](#optional-raw-api-interfaces)
- (Optional) `verify`
    - [ID token verification](#optional-id-token-verification)
- (Optional) `custom_client`
    - [HTTP client customization](#http-client-customization)
- (Optional) `oauth`
    - [OAuth 2.0 client](#optional-oauth-20-client)

## Supported APIs

Suppoted APIs of the [Firebase Auth REST API](https://firebase.google.com/docs/reference/rest/auth) are as follows:

- [x] [Exchange custom token for an ID and refresh token](https://firebase.google.com/docs/reference/rest/auth#section-verify-custom-token)
- [x] [Exchange a refresh token for an ID token](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token)
- [x] [Sign up with email / password](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password)
- [x] [Sign in with email / password](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password)
- [x] [Sign in anonymously](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-anonymously)
- [x] [Sign in with OAuth credential](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential)
- [x] [Fetch providers for email](https://firebase.google.com/docs/reference/rest/auth#section-fetch-providers-for-email)
- [x] [Send password reset email](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email)
- [ ] (Not tested) [Verify password reset code](https://firebase.google.com/docs/reference/rest/auth#section-verify-password-reset-code)
- [ ] (Not tested) [Confirm password reset](https://firebase.google.com/docs/reference/rest/auth#section-confirm-reset-password)
- [x] [Change email](https://firebase.google.com/docs/reference/rest/auth#section-change-email)
- [x] [Change password](https://firebase.google.com/docs/reference/rest/auth#section-change-password)
- [x] [Update profile](https://firebase.google.com/docs/reference/rest/auth#section-update-profile)
- [x] [Get user data](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info)
- [x] [Link with email/password](https://firebase.google.com/docs/reference/rest/auth#section-link-with-email-password)
- [x] [Link with OAuth credential](https://firebase.google.com/docs/reference/rest/auth#section-link-with-oauth-credential)
- [x] [Unlink provider](https://firebase.google.com/docs/reference/rest/auth#section-unlink-provider)
- [x] [Send email verification](https://firebase.google.com/docs/reference/rest/auth#section-send-email-verification)
- [ ] (Not tested) [Confirm email verification](https://firebase.google.com/docs/reference/rest/auth#section-confirm-email-verification)
- [x] [Delete account](https://firebase.google.com/docs/reference/rest/auth#section-delete-account)

> [!NOTE]
> Unsupported APIs have already been implemented but not tested.

## Supported OAuth ID providers

Supported OAuth ID provides are as follows:

- [ ] (Not tested) Apple (`apple.com`)
- [ ] (Not tested) Apple Game Center (`gc.apple.com`)
- [ ] (Not tested) Facebook (`facebook.com`)
- [x] GitHub (`github.com`)
- [x] Google (`google.com`)
- [ ] (Not tested) Google Play Games (`playgames.google.com`)
- [ ] (Not tested) LinkedIn (`linkedin.com`)
- [ ] (Not tested) Microsoft (`microsoft.com`)
- [ ] (Not tested) Twitter (`twitter.com`)
- [ ] (Not tested) Yahoo (`yahoo.com`)
- [ ] (Not tested) Custom (`{custom-provder-id}`)

## API Usages

Provides semantic interfaces based on a session (`fars::Session`) as following steps.

> [!IMPORTANT]
> - ID token (`fars::Session.id_token`) has expiration date.
> - API calling through a session automatically refresh an ID token by the [refresh token API](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token) when the ID token has been expired.
> - All APIs through session cosume session and return new session that has same ID token or refreshed one except for the [delete account API](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
> 
> Therefore you have to **update** session every time you use APIs through a session by returned new session.

### A usage for a siging in user

1. Create a config (`fars::Config`) with your Firebase project API key.
2. Sign in or sign up by supported options (Email & password / OAuth / Anonymous / Stored refresh token) through the config then get the session (`fars::Session`) for the siging in user.
3. Use Auth APIs for the siging in user through the session, or use ID token (`fars::Session.id_token`) for other Firebase APIs.

A sample code to [sign up with email / password](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password) and to [get user data](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info) with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:

```rust
use fars::Config;
use fars::ApiKey;
use fars::Email;
use fars::Password;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a config with your Firebase project API key.
    let config = Config::new(
        ApiKey::new("your-firebase-project-api-key"),
    );
    
    // 2. Sign up with email and password then get a session.
    let session = config.sign_up_with_email_password(
        Email::new("user@example.com"),
        Password::new("password"),
    ).await?;

    // 3. Get user data through the session and get a new session.
    let (new_session, user_data) = session.get_user_data().await?;

    // 4. Do something with new_session and user_data.

    Ok(())
}
```

### A usage for not siging in user

1. Create a config (`fars::Config`) with your Firebase project API key.
2. Use Auth APIs for a not siging in user through the config.

A sample code to [send password reset email](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email) with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:

```rust
use fars::Config;
use fars::ApiKey;
use fars::Email;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a config with your Firebase project API key.
    let config = Config::new(
        ApiKey::new("your-firebase-project-api-key"),
    );
    
    // 2. Send reset password email to specified email through the config if it has been registered.
    config.send_reset_password_email(
        Email::new("user@example"),
        None, // Option: Locale
    ).await?;

    Ok(())
}
```

## Sign in with OAuth credentials

> [!IMPORTANT]
> This crate does not provide methods to get OAuth credential for each ID provider.
>
> When you use signing in with OAuth credential, please implement a method to get target OAuth credential.

See also [supported OAuth providers](#supported-oauth-id-providers).

### Google OAuth

To sign in with Google OAuth credential, 

1. Create a config (`fars::Config`) with your Firebase project API key.
2. Get an access token of a user from Google OAuth API. See [reference](https://developers.google.com/identity/protocols/oauth2/web-server#obtainingaccesstokens).
3. Sign in with specifying `request_uri` and `IdpPostBody`.

A sample code to [sign in with Google OAuth credential](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential) with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:

```rust
use std::collections::HashMap;
use fars::Config;
use fars::ApiKey;
use fars::OAuthRequestUri;
use fars::IdpPostBody;
use fars::ProviderId

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a config with your Firebase project API key.
    let config = Config::new(
        ApiKey::new("your-firebase-project-api-key"),
    );

    // 2. Get an access token from Google OAuth by any method.
    let google_access_token = "google-access-token".to_string();

    // 3. Get a session by signing in with Google OAuth credential.
    let session = config
        .sign_in_with_oauth_credential(
            OAuthRequestUri::new("https://your-app.com/redirect/path/auth/handler"),
            IdpPostBody::new(
                ProviderId::Google, // Specify IDP
                HashMap::from([(
                    "access_token",
                    google_access_token,
                )]), // Set post body as key-value pairs.
            )?,
        )
        .await?;

    // 4. Do something with the session.

    Ok(())
}
```

## Error handling

If you handle error in this crate, please handle `fars::Result` and `fars::Error`.

> [!NOTE]
> `fars::Error::ApiError` has an error code (`fars::error::CommonErrorCode`) according to common error codes in the [API reference](https://firebase.google.com/docs/reference/rest/auth).
> You can specify error type of an API error of Firebase Auth by matching an error code (`fars::error::CommonErrorCode`).

A sample code to handle error for [signing in with email / password](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password) with [reqwest](https://github.com/seanmonstar/reqwest), [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:

```rust
use fars::Config;
use fars::ApiKey;
use fars::Email;
use fars::Password;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a config.
    let config = Config::new(
        ApiKey::new("your-firebase-project-api-key"),
    );

    // Create a session by signing in with email and password.
    match config
        .sign_in_with_email_password(
            Email::new("user@example"),
            Password::new("password"),
        )
        .await
    {
        // Success
        | Ok(session) => {
            println!(
                "Succeeded to sign in with email/password: {:?}",
                session
            );
            // Do something with the session.
            Ok(())
        },
        // Failure
        | Err(error) => {
            match error {
                // Handle HTTP request error.
                | fars::Error::HttpRequestError(error) => {
                    // Do something with HTTP request error, e.g. retry.
                    Err(error.into())
                },
                // Handle API error.
                | fars::Error::ApiError {
                    status_code,
                    error_code,
                    response,
                } => {
                    match error_code {
                        | CommonErrorCode::InvalidLoginCredentials => {
                            // Do something with invalid login credentials, e.g. display error message for user: "Invalid email or/and password.".
                            Err(fars::Error::ApiError {
                                status_code,
                                error_code,
                                response,
                            }
                            .into())
                        },
                        | CommonErrorCode::UserDisabled => {
                            // Do something with disabled user, e.g. display error message for user: "This user is disabled by administrator, please use another account.".
                            Err(fars::Error::ApiError {
                                status_code,
                                error_code,
                                response,
                            }
                            .into())
                        },
                        | CommonErrorCode::TooManyAttemptsTryLater => {
                            // Do something with too many attempts, e.g. display error message for user: "Too may requests, please try again later.".
                            Err(fars::Error::ApiError {
                                status_code,
                                error_code,
                                response,
                            }
                            .into())
                        },
                        | _ => {
                            // Do something with other API errors.
                            Err(fars::Error::ApiError {
                                status_code,
                                error_code,
                                response,
                            }
                            .into())
                        },
                    }
                },
                // Handle internal errors
                | _ => {
                    // Do something with internal errors.
                    Err(error.into())
                },
            }
        },
    }
}
```

## Raw API interfaces

Provides raw [supported APIs](#supported-apis) by `fars::api` module.

A sample code to [sign in with email / password](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password) with [reqwest](https://github.com/seanmonstar/reqwest), [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:

```rust
use fars::ApiKey;
use fars::Client;
use fars::api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Specify your API key.
    let api_key = ApiKey::new("your-firebase-project-api-key");

    // 2. Create a HTTP client.
    let client = Client::new();

    // 3. Create a request payload for the sign in API.
    let request_payload = api::SignInWithEmailPasswordRequestBodyPayload::new(
        "user@example.com".to_string(),
        "password".to_string(),
    );

    // 4. Send a request and receive a response payload of the sign in API.
    let response_payload = api::sign_in_with_email_password(
        &client,
        &api_key,
        request_payload,
    ).await?;

    // 5. Do something with the response payload.

    Ok(())
}
```

## (Optional) ID token verification

Provides ID token verification of the Firebase Auth via `fars::verification` module.

> [!NOTE]
> ID token verification is an optional feature.
> 
> Please activate this feature by CLI:
> 
> ```shell
> $ cargo add fars --features verify
> ```
> 
> or adding features to your `Cargo.toml`:
> 
> ```toml
> [dependencies]
> fars = { version = "0.2.0", features = ["verify"] }
> ```

A sample code to [verify ID token](https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library) with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:

```rust
use fars::VerificationConfig;
use fars::ProjectId;
use fars::IdToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a cofig for verification with your Firebase project ID.
    let cofing = let config = VerificationConfig::new(
        ProjectId::new("firebase-project-id"),
    );

    // Get an ID token of the Firebase Auth by any method.
    let id_token = IdToken::new("id-token");

    // Verrify the ID token.
    match config.verify_id_token(&id_token).await {
        Ok(claims) => {
            // Verification succeeded.
        },
        Err(error) => {
            // Verification failed.
        },
    }

    Ok(())
}
```

## (Optional) HTTP client customization

Provides HTTP client customization interface for Firebase Auth APIs.

> [!NOTE]
> HTTP client customization is an optional feature.
> 
> Please activate this feature by CLI:
> 
> ```shell
> $ cargo add fars --features custom_client
> ```
> 
> or adding features to your `Cargo.toml`:
> 
> ```toml
> [dependencies]
> fars = { version = "0.2.0", features = ["custom_client"] }
> ```

An example to customize timeout options of HTTP client with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:

```rust
use std::time::Duration;
use fars::Client;
use fars::ApiKey;
use fars::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a custom client with re-exported `reqwest` client.
    let client = fars::reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(10))
        .build()?;

    // 2. Customize HTTP client.
    let client = Client::custom(client);

    // 3. Create a cofig with customized client.
    let config = Config::custom(
        ApiKey::new("your-firebase-project-api-key"),
        client,
    );

    // 4. Do something with a customized config.

    Ok(())
}
```

## (Optional) OAuth 2.0 Client

TODO:

## Other examples

Please refer [/examples](./examples/) directory, [a shell script](./examples.sh) and [a study of authentication on Web frontend](https://github.com/mochi-neko/rust-frontend-playground) with [dioxus](https://github.com/DioxusLabs/dioxus).

## Changelog

See [CHANGELOG](./CHANGELOG.md).

## License

Licensed under either of the [Apache License, Version 2.0](./LICENSE-APACHE) or the [MIT](./LICENSE-MIT) license at your option.
