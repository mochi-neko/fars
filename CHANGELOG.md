# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Support OAuth2 process.
- Support `verify password reset code` on session.
- Support `confirm password reset` on session.
- Support `confirm email verification` on session.

## [0.2.0] - 2024-01-XX

### Added

- Add `LaunguageCode` to specify locale.
- Add `custom_client` feature to provide HTTP client customization interfaces.

### Changed

- (Breaking changes) Make interfaces more explicit, e.g. `"api-key".to_string()` -> `ApiKey::new("api-key")`
- Be enable to access `Seesion.refresh_token` and `Seesion.expires_in`.
- Ablish `raw` feature and raw interfaces are available by default.

## [0.1.0] - 2024-01-06

### Added

- First release.

[unreleased]: https://github.com/mochi-neko/fars/compare/v0.1.0...HEAD
[0.2.0]: https://github.com/mochi-neko/fars//compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/mochi-neko/fars/releases/tag/v0.1.0