# Changelog

All notable changes to this project will be documented in this file.

Addition of markdown posts will not be versioned or require a changelog update.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.7] - 2022-03-19

### Changed

- Updated some packages

## [0.5.6] - 2022-02-25

### Changed

- [UI 3.0] minor changes

## [0.5.5] - 2022-02-24

### Changed

- UI 3.0

## [0.5.0] - 2022-02-23

### Changed

- A lot of css changes

### Added

- `about.html` file and `/about` route

## [0.4.1] - 2021-12-29

### Changed

- [Cargo.toml] Updated `askama 0.10.5 => 0.11.0`

## [0.4.0] - 2021-12-29

### Added

- [parsers/mod.rs] Added chrono date parsing for sort function on meta vector

### Changed

- [parsers/mod.rs] Changed files `ReadDir` to a async iter stream

## [0.3.0] - 2021-12-21

### Added

- Custom error handler enum for error bubbling back to the axum handlers
