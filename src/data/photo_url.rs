/// A photo URL of a user.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct PhotoUrl {
    inner: String,
}

impl PhotoUrl {
    /// Creates a new photo URL.
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
