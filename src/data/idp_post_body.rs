use std::collections::HashMap;

use crate::Error;
use crate::ProviderId;
use crate::Result;

/// Post body for ID providers contains the OAuth credential and provider ID.
#[derive(Clone, Debug)]
pub struct IdpPostBody {
    pub(crate) query: String,
}

impl IdpPostBody {
    /// Creates a new post body for identity providers.
    ///
    /// ## Arguments
    /// - `provider_id` - The ID of the identity provider.
    /// - `credentials` - The credentials of the identity provider as hash map.
    ///
    /// ## Errors
    /// - `UrlEncodeFailed` - Failed to encode the post body as URL encoded string.
    ///
    /// ## Examples
    /// ```
    /// use std::collections::HashMap;
    /// use fars::IdpPostBody;
    /// use fars::ProviderId;
    ///
    /// let post_body = IdpPostBody::new(
    ///     ProviderId::Google,
    ///     HashMap::from([(
    ///         "access_token",
    ///         "google-access-token".to_string(),
    ///     )]),
    /// )?;
    /// ```
    pub fn new(
        provider_id: ProviderId,
        credentials: HashMap<&str, String>,
    ) -> Result<Self> {
        let mut map = HashMap::new();
        map.insert("providerId", provider_id.format());

        map.extend(credentials.clone());

        let query = serde_urlencoded::to_string(map).map_err(|error| {
            Error::UrlEncodeFailed {
                error,
            }
        })?;

        Ok(Self {
            query,
        })
    }
}
