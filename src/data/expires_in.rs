use crate::Error;
use crate::Result;

/// Expiration time in seconds of the Firebase Auth ID token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct ExpiresIn {
    inner: u64,
}

impl ExpiresIn {
    /// Parses a string into an [`ExpiresIn`].
    pub fn parse(expires_in: String) -> Result<Self> {
        Ok(Self {
            inner: expires_in
                .parse::<u64>()
                .map_err(|error| Error::ParseExpriesInFailed {
                    error,
                })?,
        })
    }

    /// Returns the inner representation.
    pub fn inner(&self) -> u64 {
        self.inner
    }
}
