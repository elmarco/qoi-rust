# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2025-07-22

### Added

- Fork the project for now (qoi -> qoicoubeh)
- encoder: add stride & various raw input formats support
- encoder: Add EncoderBuilder

### Fixed

- decode: handle the initial index edge-case before the loop
- decode: fix transparent background bug
- Fix decoding to larger output buffer
- hash_index() fix for big-endian
- Various build and clippy fixes

### Changed

- Don't run test_gen tests on big-endian
- Make tests no-std-compatible
- Make Header::{encode,decode} pub
- Bump msrv to 1.67 (due to deps)
- Bump deps 

[unreleased]: https://github.com/elmarco/qoi-rust/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/elmarco/qoi-rust/compare/v0.4.1...v0.5.0
