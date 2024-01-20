/// Refresh token of the Firebase Auth.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct RefreshToken {
    inner: String,
}

impl RefreshToken {
    /// Creates a new refresh token.
    pub fn new<S>(inner: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: inner.into(),
        }
    }

    /// Returns the inner representation.
    pub fn inner(&self) -> &str {
        &self.inner
    }
}
