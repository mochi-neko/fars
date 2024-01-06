//! Defines the post body for ID providers.

use serde::Serialize;

/// Post body for ID providers contains the OAuth credential and provider ID.
#[derive(Clone, Debug)]
pub enum IdpPostBody {
    /// Google OAuth.
    Google {
        /// OpenID Connect ID token.
        id_token: String,
    },
    /// Facebook OAuth.
    Facebook {
        access_token: String,
    },
    /// Twitter OAuth.
    Twitter {
        access_token: String,
        oauth_token_secret: String,
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
        match self {
            | IdpPostBody::Google {
                id_token,
            } => {
                let post_body = format!(
                    "id_token={id_token}&providerId=google.com",
                    id_token = id_token
                );
                serializer.serialize_str(post_body.as_str())
            },
            | IdpPostBody::Facebook {
                access_token,
            } => {
                let post_body = format!(
                    "access_token={access_token}&providerId=facebook.com",
                    access_token = access_token
                );
                serializer.serialize_str(post_body.as_str())
            },
            | IdpPostBody::Twitter {
                access_token,
                oauth_token_secret,
            } => {
                let post_body = format!(
                    "access_token={access_token}&oauth_token_secret={oauth_token_secret}&providerId=twitter.com",
                    access_token = access_token, oauth_token_secret = oauth_token_secret
                );
                serializer.serialize_str(post_body.as_str())
            },
        }
    }
}
