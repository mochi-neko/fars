/// OAuth continue URI.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct OAuthContinueUri {
    inner: String,
}

impl OAuthContinueUri {
    /// Creates a new OAuth continue URI.
    pub fn new<S>(inner: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: inner.into(),
        }
    }

    /// Returns the inner representation.
    pub(crate) fn inner(&self) -> &str {
        &self.inner
    }
}
