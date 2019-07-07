# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased](https://github.com/atroche/rust-headless-chrome/compare/v0.3.0...HEAD)

### Added
### Removed
### Changed

## 0.3.0 - 2019-07-07

### Added

* Re-export Element struct in top level module
* Better crate-level docs, and also docs for the Element struct
* Browser::default convenience method for quickly getting a headless browser with default options

### Removed
### Changed

## 0.2.0 - 2019-07-07

Note: starting with this release we're going to bump the minor version whenever anything new is added to the public API.

### Added

* [Response handling callback for tabs (`Tab.enable_response_handler`)](https://github.com/atroche/rust-headless-chrome/pull/133)

### Removed
### Changed

* [Fixed a race condition in Tab.wait_until_navigated](https://github.com/atroche/rust-headless-chrome/pull/135)
* [Bump dependencies (rand, ureq, directories) and remove base64 dev dep](https://github.com/atroche/rust-headless-chrome/pull/134)


## 0.1.5 - 2019-06-19

### Added

* [Tab.get_script_source, Tab.enable_debugger, Tab.disable_debugger](https://github.com/atroche/rust-headless-chrome/commit/625c59f9957d3ffa1853164d1d77e9c252d116ee)
* [Add ability to set window size of browser on launch](https://github.com/atroche/rust-headless-chrome/pull/123)
* [Scroll elements into view before clicking, mousing over or focusing on them](https://github.com/atroche/rust-headless-chrome/pull/128)
* [FrameTree.child_frames field](https://github.com/atroche/rust-headless-chrome/commit/9c86817fdbf8fa63620cad3700f7063781335d20)
* [When waiting for elements, return most errors early instead of retrying and timing out](https://github.com/atroche/rust-headless-chrome/pull/129)
* [Add `await_promise` argument to Tab.call_js_fn](https://github.com/atroche/rust-headless-chrome/commit/d82ffa8fd4c3efaed1721d8721068d2c6d6c7c9c)
* [Search for existing Chrome / Chromium binaries in more locations](https://github.com/atroche/rust-headless-chrome/pull/126/files)

### Removed

* [Remove some out-dated examples, along with a couple of dependencies](https://github.com/atroche/rust-headless-chrome/commit/7e99bb861bf8476192b6402a12e9c7d06f15911f)

### Changed

* [Fix Windows build](https://github.com/atroche/rust-headless-chrome/pull/118)
* [Use ureq instead of reqwest for fetching Chromium binaries](https://github.com/atroche/rust-headless-chrome/commit/acf336707759b646f59d68b05465a0e0ef2a0fa7)


## 0.1.4 - 2019-03-21

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
