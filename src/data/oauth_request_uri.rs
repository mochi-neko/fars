/// OAuth request URI.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct OAuthRequestUri {
    inner: String,
}

impl OAuthRequestUri {
    /// Creates a new OAuth request URI.
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
