//! An example to handle error with signing in.
//!
//! ```shell
//! $ cargo run --example handle_error -- --email <email> --password <password>
//! ```

use clap::Parser;
use fars::{error::CommonErrorCode, Config};

#[derive(Parser)]
struct Credentials {
    #[arg(short, long)]
    email: String,
    #[arg(short, long)]
    password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments.
    let credentials = Credentials::parse();

    // Read API key from the environment variable.
    let api_key = std::env::var("FIREBASE_API_KEY")?;

    // Create a config.
    let config = Config::new(api_key);

    // Create a session by signing in with email and password.
    match config
        .sign_in_with_email_password(
            credentials.email.clone(),
            credentials.password.clone(),
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
                    println!("HTTP request error: {:?}", error);
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
                            eprintln!("Invalid email and/or password.");
                            // Do something with invalid login credentials, e.g. display error message for user.
                            Err(fars::Error::ApiError {
                                status_code,
                                error_code,
                                response,
                            }
                            .into())
                        },
                        | CommonErrorCode::UserDisabled => {
                            eprintln!("This user is disabled.");
                            // Do something with disabled user, e.g. display error message for user.
                            Err(fars::Error::ApiError {
                                status_code,
                                error_code,
                                response,
                            }
                            .into())
                        },
                        | CommonErrorCode::TooManyAttemptsTryLater => {
                            eprintln!("Too many attempts, try again later.");
                            // Do something with too many attempts, e.g. display error message for user.
                            Err(fars::Error::ApiError {
                                status_code,
                                error_code,
                                response,
                            }
                            .into())
                        },
                        | _ => {
                            eprintln!(
                                "API error: ({:?}) {:?} - {:?}",
                                status_code, error_code, response
                            );
                            // Do something with other errors.
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
                    eprintln!("Internal error: {:?}", error);
                    // Do something with internal errors.
                    Err(error.into())
                },
            }
        },
    }
}
