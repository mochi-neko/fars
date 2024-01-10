//! Defines the email of an user.

/// Email of an user.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Email {
    pub(crate) inner: String,
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
}
