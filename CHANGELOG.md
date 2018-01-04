# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## unreleased

### Changed

* Generate bindings using `bindgen` at compile time, rather than checked into
  repo. This also added use of the bindgen option `constified_enum_module` which
  changes the API of the bindings slightly.

## [0.1.0] - 2017-10-03

### Added

* Initial release
