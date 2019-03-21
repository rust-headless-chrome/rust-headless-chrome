# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased](https://github.com/atroche/rust-headless-chrome/compare/v0.1.4...HEAD)
### Added

* Tab.get_script_source, Tab.enable_debugger, Tab.disable_debugger

### Removed
### Changed


## 0.1.4 - 2018-03-21

### Added
* [Tab.capture_screenshot](https://github.com/atroche/rust-headless-chrome/pull/48)
* [Tab.print_to_pdf](https://github.com/atroche/rust-headless-chrome/pull/107)
* [Element.wait_for_elements](https://github.com/atroche/rust-headless-chrome/pull/90)
* [Automatic downloading of Chromium binary for people who don't want to use their own binary](https://github.com/atroche/rust-headless-chrome/pull/83)
* [Tab.reload](https://github.com/atroche/rust-headless-chrome/pull/49)
* [Network request interception](https://github.com/atroche/rust-headless-chrome/pull/98)
* [Method chaining on Tab for common methods like click()](https://github.com/atroche/rust-headless-chrome/pull/44)
* [Browser.new_tab](https://github.com/atroche/rust-headless-chrome/pull/56)
* [Incognito support (Browser.new_context)](https://github.com/atroche/rust-headless-chrome/pull/97)
* [Element.capture_screenshot](https://github.com/atroche/rust-headless-chrome/pull/59)
* [Element.get_box_model](https://github.com/atroche/rust-headless-chrome/pull/67)
* [Support for preloading extensions](https://github.com/atroche/rust-headless-chrome/pull/69)
* [Support for watching JS / CSS coverage](https://github.com/atroche/rust-headless-chrome/pull/86)
* [Element.move_mouse_over and Tab.move_mouse_to_point functions](https://github.com/atroche/rust-headless-chrome/pull/96)
* [Browser.get_version](https://github.com/atroche/rust-headless-chrome/pull/66)
* [LaunchOptionsBuilder](https://github.com/atroche/rust-headless-chrome/pull/62)
* Added badge to [the Discord server](https://discord.gg/yyGEzcc) in README

### Changed
* [Renamed cdtp module to protocol](https://github.com/atroche/rust-headless-chrome/pull/80)
* [Refactored Waiting helper](https://github.com/atroche/rust-headless-chrome/pull/88)
* [Exposed more modules (like Browser, Tab and Element) as public](https://github.com/atroche/rust-headless-chrome/pull/70)
* [protocol::dom::Node.attributes is now a HashMap](https://github.com/atroche/rust-headless-chrome/pull/52/files)
* Run Travis on stable and nightly on Linux and MacOS, with rustfmt and pedantic clippy.
* Fixed [some  concurrency issues](https://github.com/atroche/rust-headless-chrome/pull/41)

### Removed
* [Removed Element's found_via_selector field](https://github.com/atroche/rust-headless-chrome/pull/101/files)
