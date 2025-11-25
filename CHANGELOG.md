# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

* Document all features on docs.rs ([#23](https://github.com/vbrandl/actix-request-identifier/pull/23))

### Dependencies
- Bump `rustsec/audit-check` from 1 to 2 ([#24](https://github.com/vbrandl/actix-request-identifier/pull/24))
- Bump `dangoslen/dependabot-changelog-helper` from 3 to 4 ([#25](https://github.com/vbrandl/actix-request-identifier/pull/25))
- Bump `stefanzweifel/git-auto-commit-action` from 5 to 7 ([#26](https://github.com/vbrandl/actix-request-identifier/pull/26), [#28](https://github.com/vbrandl/actix-request-identifier/pull/28))
- Bump `actions/checkout` from 4 to 6 ([#27](https://github.com/vbrandl/actix-request-identifier/pull/27), [#29](https://github.com/vbrandl/actix-request-identifier/pull/29))

## [4.2.0] 2024-04-13

* Optional feature to generate UUIDv7 request IDs ([#20](https://github.com/vbrandl/actix-request-identifier/pull/20))

### Dependencies
- Bump `actions/checkout` from 1 to 4 (#12, [#16](https://github.com/vbrandl/actix-request-identifier/pull/16))
- Bump `actions/cache` from 1 to 4 (#13, [#18](https://github.com/vbrandl/actix-request-identifier/pull/18))
- Bump `stefanzweifel/git-auto-commit-action` from 4 to 5 ([#17](https://github.com/vbrandl/actix-request-identifier/pull/17))

## [4.1.0] 2023-01-13

* Breaking: Allow reusing an incoming request id, supplied via the request header ([#7])

  This changes the default behavior of the crate and is considered a breaking change.

[#7]: https://github.com/vbrandl/actix-request-identifier/pull/7
