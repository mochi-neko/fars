# fars

An unofficial Rust client for the [Firebase Auth REST API](https://firebase.google.com/docs/reference/rest/auth) with [reqwest](https://github.com/seanmonstar/reqwest) backend.

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

Supported OAuth ID provides 

- [ ] Apple (`apple.com`)
- [ ] Apple Game Center (`gc.apple.com`)
- [ ] (Not tested) Facebook (`facebook.com`)
- [ ] GitHub (`github.com`)
- [x] Google (`google.com`)
- [ ] Google Play Games (`playgames.google.com`)
- [ ] LinkedIn (`linkedin.com`)
- [ ] Microsoft (`microsoft.com`)
- [ ] (Not tested) Twitter (`twitter.com`)
- [ ] Yahoo (`yahoo.com`)

> [!NOTE]
> Unsupported providers have either not been tested or the format of `IdpPostBody` is not documented.

## Usages

You can select usage from two options:

1. Raw API interfaces
2. Session-based interfaces

### 1. Raw API interfaces

You can use raw [supported APIs](#supported-apis) by `fars::api` module.

Please refer each document, API reference and examples.

A sample code to [sign in with email / password](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-email-password) with [reqwest](https://github.com/seanmonstar/reqwest), [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:

```rust
use fars::api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Specify your API key.
    let api_key = "your-firebase-project-api-key".to_string();

    // Create a reqwest client.
    let client = reqwest::Client::new();

    // Create a request payload for the sign in API.
    let request_payload = api::SignInWithEmailPasswordRequestBodyPayload::new(
        "user@example.com".to_string(),
        "password".to_string(),
    );

    // Send a request and receive a response payload of the sign in API.
    let response_payload = api::sign_in_with_email_password(
        client,
        api_key,
        request_payload,
    ).await?;

    // Do something with the response payload.

    Ok(())
}
```

### 2. Session-based interfaces

You can use semantic interfaces based on a session (`fars::Session`) as following steps.

> [!IMPORTANT]
> 1. ID token (`fars::Session.id_token`) has expiration date.
> 2. API calling through a session automatically refresh an ID token by the [refresh token API](https://firebase.google.com/docs/reference/rest/auth#section-refresh-token) when the ID token has been expired.
> 3. All APIs through session cosume session and return new session that has same ID token or refreshed one except for the [delete account API](https://firebase.google.com/docs/reference/rest/auth#section-delete-account).
> 4. Therefore you have to **update** session every time you use APIs through a session by returned new session.

#### 2-1. A usage for logged in user

1. Create a config (`fars::Config`) with your Firebase project API key.
2. Sign in or sign up by supported options (Email & password / OAuth / Anonymous / Stored refresh token) through the config then get the session (`fars::Session`) for logged in user.
3. Use Auth APIs for logged in user through the session, or use ID token (`fars::Session.id_token`) for other Firebase APIs.

A sample code to [sign up with email / password](https://firebase.google.com/docs/reference/rest/auth#section-create-email-password) and to [get user data](https://firebase.google.com/docs/reference/rest/auth#section-get-account-info) with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:

```rust
use fars::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a config.
    let config = Config::new(
        "your-firebase-project-api-key".to_string(),
    );
    
    // Sign up with email and password then get a session.
    let session = config.sign_up_with_email_password(
        "user@example.com".to_string(),
        "password".to_string(),
    ).await?;

    // Get user data through the session and get a new session.
    let (new_session, user_data) = session.get_user_data().await?;

    // Do something with new_session and user_data.

    Ok(())
}
```

#### 2-2. A usage for not logged in user

1. Create a config (`fars::Config`) with your Firebase project API key.
2. Use Auth APIs for not logged in user through the config.

A sample code to [send password reset email](https://firebase.google.com/docs/reference/rest/auth#section-send-password-reset-email) with [tokio](https://github.com/tokio-rs/tokio) and [anyhow](https://github.com/dtolnay/anyhow) is as follows:

```rust
use fars::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a config.
    let config = Config::new(
        "your-firebase-project-api-key".to_string(),
    );
    
    // Send reset password email to specified email through the config if it has been registered.
    config.send_reset_password_email(
        "user@example".to_string(),
        None, // Option: Locale
    ).await;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a config.
    let config = Config::new("your-firebase-project-api-key".to_string());

    // Create a session by signing in with email and password.
    match config
        .sign_in_with_email_password(
            "user@example".to_string(),
            "password".to_string,
        )
        .await
    {
        // Success
        | Ok(_session) => {
            println!(
                "Succeeded to sign in with email/password: {}",
                credentials.email
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
                // Internal errors
                | _ => {
                    // Do something with internal errors.
                    Err(error.into())
                },
            }
        },
    }
}
```

## Todo for pulishing

- [x] Review error types and descriptions.
- [x] Write error handling documents.
- [x] Add errors documents.
- [x] Review re-export moudles.
- [ ] Check reqwest accessibility.
- [ ] Check `Cargo.toml` settings.
- [ ] Write some examples to `./examples/`.
- [ ] Update changelog.
- [ ] Check no panic.
- [ ] Consider decoding ID token.
- [ ] Add CI to build.
- [ ] Check documentation format.
- [ ] Check examples in documents.


## Changelog

See [CHANGELOG](./CHANGELOG.md).

## License

Licensed under either of the [Apache License, Version 2.0](./LICENSE-APACHE) or the [MIT](./LICENSE-MIT) license at your option.
