/// OAuth request URI.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct OAuthRequestUri {
    inner: String,
}

impl OAuthRequestUri {
    /// Creates a new OAuth request URI.
    pub fn new<S>(into: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: into.into(),
        }
    }

    /// Returns the inner representation.
    pub fn inner(&self) -> &str {
        &self.inner
    }
}
