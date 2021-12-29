# Changelog

All notable changes to this project will be documented in this file.

Addition of markdown posts will not be versioned or require a changelog update.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2021-12-29

### Added

- [parsers/mod.rs] Added chrono date parsing for sort function on meta vector

### Changed

- [parsers/mod.rs] Changed files `ReadDir` to a async iter stream

## [0.3.0] - 2021-12-21

### Added

- Custom error handler enum for error bubbling back to the axum handlers