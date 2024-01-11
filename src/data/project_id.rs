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

    /// Returns the inner representation.
    pub fn inner(&self) -> &str {
        &self.inner
    }
}
