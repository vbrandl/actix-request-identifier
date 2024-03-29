# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


### Dependencies
- Bump `actions/checkout` from 1 to 4 (#12, [#16](https://github.com/vbrandl/actix-request-identifier/pull/16))
- Bump `actions/cache` from 1 to 4 (#13, [#18](https://github.com/vbrandl/actix-request-identifier/pull/18))
- Bump `stefanzweifel/git-auto-commit-action` from 4 to 5 ([#17](https://github.com/vbrandl/actix-request-identifier/pull/17))

## [4.1.0] 2023-01-13

* Breaking: Allow reusing an incoming request id, supplied via the request header ([#7])

  This changes the default behavior of the crate and is considered a breaking change.

[#7]: https://github.com/vbrandl/actix-request-identifier/pull/7