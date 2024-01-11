/// Password of an user.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Password {
    inner: String,
}

impl Password {
    /// Creates a new password.
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
