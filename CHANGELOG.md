# Changelog

## Unreleased

## 0.4.2 - 2024-11-18

* Added `Plugin::receive_message` function, to get messages from X-Plane or other plugins [#22](https://github.com/samcrow/rust-xplm/pull/22)

## 0.4.1 - 2024-04-21

* Added versions module with a wrapper for `XPLMGetVersions`

## 0.4.0 - 2024-03-13

* Updated xplm-sys dependency to 0.5.0
* Changed from the `quick-error` crate to the more maintained derive macro `thiserror` crate
* Implemented the `debug!` and `debugln!` macros (same usage as `print!`/`println!`)
* Marked the `debug()` function as deprecated and changed all usages over to the new macros
* Renamed the examples and adjusted their code
* Shortened the minimal example (easier to understand for newcomers)
* Updated the README again, mainly the example and status

* README badges for easy access to the docs etc.
* Flatten the module structure: e.g. `plugin/mod.rs` into `plugin.rs`
* Some refactoring for readability
* Project formatting
* Updated deprecated code
* Removed editor specific config files

## 0.3.1 - 2020-05-31

* Updated dependency xplm-sys to 0.4.0
* Updated dependency quick-error to 1.2.3
