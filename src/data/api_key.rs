/// The Firebase project API key.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ApiKey {
    inner: String,
}

impl ApiKey {
    /// Creates a new API key.
    pub fn new<S>(into: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: into.into(),
        }
    }

    pub(crate) fn inner(&self) -> &str {
        &self.inner
    }
}
