# X-Plane plugin APIs for Rust

[![Crates.io Version](https://img.shields.io/crates/v/xplm.svg)](https://crates.io/crates/xplm)
[![Documentation](https://docs.rs/xplm/badge.svg)](https://docs.rs/xplm)
[![License](https://img.shields.io/crates/l/xplm.svg)](https://github.com/samcrow/rust-xplm#license)

## Purpose

**Rust XPLM** provides a convenient interface for X-Plane plugin development in the Rust programming language for all
platforms.

As we use the [X-Plane SDK](https://developer.x-plane.com/sdk/) version 3.0, any plugin created with this library
supports X-Plane version 11.10 or later.

## Status

The library is still in an incomplete state. As a result some parts of the SDK may only be sparsely covered or missing
completely.

- [x] Compiles and is callable from X-Plane
- [x] Debug logging to the console / log file
- [x] DataRef reading and writing
- [x] Commands
- [ ] GUI - Needs further work
- [ ] Drawing - Needs further work

## Example

Some more examples can be found in the `examples/` directory.

This small snippet is the minimal boilerplate needed to make your plugin compile.

```rust
extern crate xplm;

use xplm::plugin::{Plugin, PluginInfo};
use xplm::{debugln, xplane_plugin};

struct MinimalPlugin;

impl Plugin for MinimalPlugin {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        // The following message should be visible in the developer console and the Log.txt file
        debugln!("Hello, World! From the Minimal Rust Plugin");
        Ok(MinimalPlugin)
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from("Minimal Rust Plugin"),
            signature: String::from("org.samcrow.xplm.examples.minimal"),
            description: String::from("A plugin written in Rust"),
        }
    }
}

xplane_plugin!(MinimalPlugin);
```

### Compiling and installing a plugin

```bash
cargo new --lib my-rxplm-project
cd my-rxplm-project
cargo add xplm
```

Then add to `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]
```

Copy minimal example from above into `src/lib.rs`

`cargo build`

Rename `target/debug/my_rxplm_project.dll` to `win.xpl` (or `my_rxplm_project.so` to `lin.xpl`, etc) and copy to the aircraft/scenery/sim plugins folder

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall
be dual licensed as above, without any additional terms or conditions.
