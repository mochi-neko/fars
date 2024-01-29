/// The issuer that controls who can sign in to the application for Microsoft OAuth 2.0.
///
/// See also [the official document of endpoint](https://learn.microsoft.com/en-us/entra/identity-platform/v2-protocols#endpoints).
pub enum MicrosoftIssuer {
    /// For both Microsoft accounts and work or school accounts.
    Common,
    /// For work or school accounts only.
    Organizations,
    /// For Microsoft accounts only.
    Consumers,
    /// For the tenant ID or domain name.
    Tenant {
        tenant_id: String,
    },
}

impl MicrosoftIssuer {
    pub(crate) fn format(&self) -> &str {
        match self {
            | Self::Common => "common",
            | Self::Organizations => "organizations",
            | Self::Consumers => "consumers",
            | Self::Tenant {
                tenant_id,
            } => tenant_id.as_str(),
        }
    }
}
