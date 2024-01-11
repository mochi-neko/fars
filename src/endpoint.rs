//! Endpoint for the Firebase Auth REST API.
//!
//! See also [API reference](https://firebase.google.com/docs/reference/rest/auth).

/// The endpoint to send the request to.
///
/// See also [API reference](https://firebase.google.com/docs/reference/rest/auth).
pub(crate) enum Endpoint {
    /// accounts:signInWithCustomToken
    SignInWithCustomToken,
    /// token
    Token,
    /// accounts:signUp
    SignUp,
    /// accounts:signInWithPassword
    SignInWithPassword,
    /// accounts:signInWithIdp
    SignInWithIdp,
    /// accounts:createAuthUri
    CreateAuthUri,
    /// accounts:sendOobCode
    SendOobCode,
    /// accounts:resetPassword
    ResetPassword,
    /// accounts:update
    Update,
    /// accounts:lookup
    Lookup,
    /// accounts:delete
    Delete,
}

impl Endpoint {
    /// Formats the endpoint to a string.
    pub(crate) fn format(self) -> &'static str {
        match self {
            | Endpoint::SignInWithCustomToken => {
                "accounts:signInWithCustomToken"
            },
            | Endpoint::Token => "token",
            | Endpoint::SignUp => "accounts:signUp",
            | Endpoint::SignInWithPassword => "accounts:signInWithPassword",
            | Endpoint::SignInWithIdp => "accounts:signInWithIdp",
            | Endpoint::CreateAuthUri => "accounts:createAuthUri",
            | Endpoint::SendOobCode => "accounts:sendOobCode",
            | Endpoint::ResetPassword => "accounts:resetPassword",
            | Endpoint::Update => "accounts:update",
            | Endpoint::Lookup => "accounts:lookup",
            | Endpoint::Delete => "accounts:delete",
        }
    }
}
