# Changelog

All notable changes to this project will be documented in this file.

Addition of markdown posts will not be versioned or require a changelog update.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.12.1] - 2022-08-02

## Added

- Put webpack and cargo build commands in a yarn script

## [0.12.0] - 2022-08-02

## Added

- added cloudflare front-end analytics | probably will get blocked by a lot of clients though

## Changed

- continued fine tuning of the webpack workflow

## [0.11.1] - 2022-07-22

## Added

- more work on the `/admin` spa with `webpack` and `preact` | not sure if a spa will be the final build idea for the admin page

## [0.11.0] - 2022-07-17

## Added

- node and webpack structure for handling some frontend with `Preact` | this notes a large structure change for the site, future versions may have more hybrid hydration on other routes | there is consideration over handling everything in `Preact` but the static blog content, but this would be a major overhaul

## Changed

- adjusted `/admin` routes to migrate over to being handled by `Preact`

## [0.10.2] - 2022-07-12

## Added

- started work on `/admin` routes | may switch to a frontend SPA for this route

## [0.10.1] - 2022-07-10

## Changed

- updated dependencies

## [0.10.0] - 2022-07-09

### Added

- starting work on admin authentication | brought in jwt functions

### Changed

- adjust all imports to the same style | external then internal

## [0.9.2] - 2022-07-03

### Changed

- Adjusted all templates to use `.html.j2` | this allows for better IDE syntax highlighting support

## [0.9.1] - 2022-06-14

### Changed

- `@supports (-webkit-touch-callout: none) and (not (translate: none))` added to `main.css` this allows for custom alternate support for browsers that don't support `flex gap` like older version of ios

## [0.9.0] - 2022-06-12

### Added

- Owl svg logo

### Changed

- A lot of color design was changed
- Moved tags out of main anchor tag | will probably get changed again

### Removed

- Theme switcher was removed for now as its very WIP | all the pieces are there just need some work frontend work

## [0.8.8] - 2022-05-25

### Changed

- `/parsers/mod.rs` | moved markdown file extension and file path strings to constants for better visibility in the code
- `[ui 3.1]` | changed to a transparent navbar and moved theme switcher out | currently broken right now

## [0.8.7] - 2022-05-21

### Changed

- `mod parsers` changed parser functions to take the markdown files content as a parameter instead of the filename, calls to the file itself only happen once now then parsed to html and `Meta`. This chamges IO calls from happening twice per file on load, to only once now.

## [0.8.6] - 2022-05-18

### Changed

- `/public/js/theme.js` | moved script to execute at the bottom on `<body>` and moved the logic out of DOM load event; allows for quicker execution of the localStorage check
- Adjusted some of the css styling; the 'light' theme still needs to be done correctly [wip]

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
