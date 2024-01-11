/// ID token of the Firebase Auth.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct IdToken {
    inner: String,
}

impl IdToken {
    /// Creates a new ID token.
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
