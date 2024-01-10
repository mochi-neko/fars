use serde::Serialize;

use crate::ProviderId;

/// Post body for ID providers contains the OAuth credential and provider ID.
#[derive(Clone, Debug)]
pub enum IdpPostBody {
    /// Apple OAuth.
    ///
    /// Refers to the [document](https://developer.apple.com/documentation/devicemanagement/user_enrollment/onboarding_users_with_account_sign-in/implementing_the_oauth2_authentication_user-enrollment_flow).
    Apple {
        access_token: String,
    },
    /// Google OAuth.
    ///
    /// Refers to the [document](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential).
    Google {
        /// OpenID Connect ID token.
        id_token: String,
    },
    /// Facebook OAuth.
    ///
    /// Refers to the [document](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential).
    Facebook {
        access_token: String,
    },
    /// Twitter OAuth.
    ///
    /// Refers to the [document](https://firebase.google.com/docs/reference/rest/auth#section-sign-in-with-oauth-credential).
    Twitter {
        access_token: String,
        oauth_token_secret: String,
    },
    /// GitHub OAuth.
    ///
    /// Refers to the [document](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps).
    GitHub {
        access_token: String,
    },
    /// Microsoft OAuth.
    ///
    /// Refers to the [document](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow).
    Microsoft {
        access_token: String,
    },
    /// Yahoo OAuth.
    ///
    /// Refers to the [document](https://developer.yahoo.com/oauth2/guide/flows_authcode/).
    Yahoo {
        access_token: String,
    },
    /// LinkedIn OAuth.
    ///
    /// Refers to the [document](https://www.linkedin.com/pulse/oauth2-amit-nadiger/).
    LinkedIn {
        access_token: String,
    },
}

impl Serialize for IdpPostBody {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let post_body = match self {
            | IdpPostBody::Apple {
                access_token,
            } => {
                format!(
                    "access_token={}&providerId={}",
                    access_token,
                    ProviderId::Apple.format(),
                )
            },
            | IdpPostBody::Google {
                id_token,
            } => {
                format!(
                    "id_token={}&providerId={}",
                    id_token,
                    ProviderId::Google.format(),
                )
            },
            | IdpPostBody::Facebook {
                access_token,
            } => {
                format!(
                    "access_token={}&providerId={}",
                    access_token,
                    ProviderId::Facebook.format(),
                )
            },
            | IdpPostBody::Twitter {
                access_token,
                oauth_token_secret,
            } => {
                format!(
                    "access_token={}&oauth_token_secret={}&providerId={}",
                    access_token,
                    oauth_token_secret,
                    ProviderId::Twitter.format(),
                )
            },
            | IdpPostBody::GitHub {
                access_token,
            } => {
                format!(
                    "access_token={}&providerId={}",
                    access_token,
                    ProviderId::GitHub.format(),
                )
            },
            | IdpPostBody::Microsoft {
                access_token,
            } => {
                format!(
                    "access_token={}&providerId={}",
                    access_token,
                    ProviderId::Microsoft.format(),
                )
            },
            | IdpPostBody::Yahoo {
                access_token,
            } => {
                format!(
                    "access_token={}&providerId={}",
                    access_token,
                    ProviderId::Yahoo.format(),
                )
            },
            | IdpPostBody::LinkedIn {
                access_token,
            } => {
                format!(
                    "access_token={}&providerId={}",
                    access_token,
                    ProviderId::LinkedIn.format(),
                )
            },
        };

        serializer.serialize_str(post_body.as_str())
    }
}
