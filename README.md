
# X-Plane plugin APIs for Rust

## Purpose

This library provides convenient Rust interfaces to the [X-Plane plugin APIs](http://www.xsquawkbox.net/xpsdk/mediawiki/Main_Page).

With this library and the [xplane_plugin](https://crates.io/crates/xplane_plugin)
crate, X-Plane plugins can be easily developed in Rust.

## Status

This library is still incomplete, but some parts are relatively stable. The status
is listed below:

* `data` module: Complete, relatively stable
* `graphics` module: Unstable
* `ui` module: Unstable, likely to be significantly restructured
* `command` module: Unstable
* `features` module: Stable
* `flight_loop` module: Unstable
* `frequency` module: Stable
* `nav` module: Unstable
* `position` module: Unstable
* `terrain` module: Unstable

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
