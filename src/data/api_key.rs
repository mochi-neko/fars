use std::env::VarError;

/// The Firebase project API key.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ApiKey {
    inner: String,
}

impl ApiKey {
    /// Creates a new API key.
    pub fn new<S>(inner: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: inner.into(),
        }
    }

    /// Loads API key from environment variable: `"FIREBASE_API_KEY"`.
    pub fn from_env() -> std::result::Result<Self, VarError> {
        let key = std::env::var("FIREBASE_API_KEY")?;

        Ok(Self::new(key))
    }

    pub(crate) fn inner(&self) -> &str {
        &self.inner
    }
}
