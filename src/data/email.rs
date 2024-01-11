/// Email of an user.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Email {
    inner: String,
}

impl Email {
    /// Creates a new email.
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
