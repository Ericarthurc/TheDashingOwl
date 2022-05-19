# Changelog

All notable changes to this project will be documented in this file.

Addition of markdown posts will not be versioned or require a changelog update.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.5] - 2022-05-18

### Added

- `/public/js/theme.js` | JavaScript handler for getting/setting localStorage key `data-theme`
- `/templates/inclues/theme_switcher.html`
- Added css implementation for theme switcher input [wip] [transitions cause flashing on load]

### Changed

- Updated cargo packges 


## [0.8.0] - 2022-05-06

### Added

- handler_404 template [wip]

### Changed

- Public assets now route to `./public`
- sw.js and robots.txt are still served on `/`
- Fallback now works correctly with tower-http 3^

### Removed

- tower-http fallback route

## [0.7.2] - 2022-05-06
### - Will not compile

### Changed

- Updated packages
- tower-http 3 update broke the fallback handler 

## [0.7.1] - 2022-04-09

### Added

- The intro markdown post

### Changed

- Site moving to `Beta` production
- Adjusted blog header colors
- Adjusted blog-index sizing

### Removed

- All the dummy markdown files

## [0.7.0] - 2022-04-05

### Changed

- Updated to axum `0.5.1`
- Finished meta parser for `category`
- Updated html/css on `/series` and `category` to match main index for now

## [0.6.0] - 2022-03-20

### Added

- Starting ground work for `/category/:category` route

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
