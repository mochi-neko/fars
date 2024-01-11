/// The Firebase project ID.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ProjectId {
    inner: String,
}

impl ProjectId {
    /// Creates a new project ID.
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
