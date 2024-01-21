use std::env::VarError;

/// The Firebase project ID.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ProjectId {
    inner: String,
}

impl ProjectId {
    /// Creates a new project ID.
    pub fn new<S>(inner: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: inner.into(),
        }
    }

    /// Loads API key from environment variable: `"FIREBASE_PROJECT_ID"`.
    pub fn from_env() -> std::result::Result<Self, VarError> {
        let id = std::env::var("FIREBASE_PROJECT_ID")?;

        Ok(Self::new(id))
    }

    /// Returns the inner representation.
    pub fn inner(&self) -> &str {
        &self.inner
    }
}
