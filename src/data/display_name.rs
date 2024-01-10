/// A display name of a user.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct DisplayName {
    pub(crate) inner: String,
}

impl DisplayName {
    /// Creates a new display name.
    pub fn new<S>(into: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: into.into(),
        }
    }
}
