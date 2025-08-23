# Changelog

This file documents the changes made to the formatter with each release. This project uses [semantic versioning](https://semver.org/spec/v2.0.0.html).

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
