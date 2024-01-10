/// OAuth request URI.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct OAuthRequestUri {
    pub(crate) inner: String,
}

impl OAuthRequestUri {
    /// Creates a new OAuth request URI.
    pub fn new<S>(into: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            inner: into.into(),
        }
    }
}
