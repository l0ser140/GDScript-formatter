# Changelog

This file documents the changes made to the formatter with each release. This project uses [semantic versioning](https://semver.org/spec/v2.0.0.html).

## Release 0.4.0 (2025-09-10)

### Fixed

- Trailing comments at the end of functions were being wrapped on a new line. They're now preserved at the end of the function line.

### Changed

- Updated to latest version of the GDScript parser with adapted queries for new body node in setters and getters
- Added test case for trailing comments at the end of functions to ensure correct formatting

## Release 0.3.0 (2025-09-04)

### Added

- Print the help message if there are no arguments or piped input

### Fixed

- Semicolons: wrap statements on multiple lines when needed, preserve indentation in code blocks
- Inline comments after colons wrapping on another line

### Changed

- Make tests run much 3 to 4x faster and greatly improve output diff
- Use cargo configuration to strip debug symbols from release binaries

## Release 0.2.0 (2025-08-23)

### Added

- Support for multi-line wrapping of function parameters with extra indentation
- Spacing around the "as" keyword

### Changed

- Formatter now overwrites formatted files by default instead of outputting to stdout
- Added option to output to stdout when needed
- Version number is now read directly from Cargo.toml at build time

## Release 0.1.0 (2025-08-21)

This is the initial release of the GDScript formatter.

### Added

- Support for many GDScript formatting rules:
  - Consistent spacing between operators, keywords, and after commas in most cases
  - Single and multi-line formatting for arrays and dictionaries
  - Consistent indentation for blocks, function definitions, and control structures
  - Enforces blank lines between functions and classes
- Configuration option for indentation (spaces or tabs)
- Test suite with input/expected file pairs (run with `cargo test`)
- Cross-platform support (Linux, macOS, Windows) and automated builds with GitHub Actions
