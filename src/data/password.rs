//! Defines the password of an user.

/// Password of an user.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Password {
    pub(crate) inner: String,
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
}
