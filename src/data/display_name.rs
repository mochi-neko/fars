/// A display name of a user.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct DisplayName {
    inner: String,
}

impl DisplayName {
    /// Creates a new display name.
    pub fn new<S>(inner: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: inner.into(),
        }
    }

    pub(crate) fn inner(&self) -> &str {
        &self.inner
    }
}
