/// A photo URL of a user.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct PhotoUrl {
    inner: String,
}

impl PhotoUrl {
    /// Creates a new photo URL.
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
