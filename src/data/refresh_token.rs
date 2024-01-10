//! Defines the Refresh token of the Firebase Auth.

/// Refresh token of the Firebase Auth to exchange for a new ID token and refresh token.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct RefreshToken {
    pub(crate) inner: String,
}

impl RefreshToken {
    /// Creates a new refresh token.
    pub fn new<S>(into: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: into.into(),
        }
    }
}
