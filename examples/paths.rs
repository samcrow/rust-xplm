extern crate xplm;

use xplm::paths::{aircraft_path, plugin_path, prefs_path, system_path};
use xplm::plugin::{Plugin, PluginInfo};
use xplm::{debugln, xplane_plugin};

struct PathsDemo;

impl Plugin for PathsDemo {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        debugln!(
            "[Rust-XPLM] X-Plane root folder: {}",
            system_path().unwrap().display()
        );
        debugln!(
            "[Rust-XPLM] X-Plane prefs file: {}",
            prefs_path().unwrap().display()
        );
        debugln!(
            "[Rust-XPLM] Plugin path: {}",
            plugin_path().unwrap().display()
        );
        debugln!(
            "[Rust-XPLM] Aircraft path: {}",
            aircraft_path().unwrap().display()
        );

        Ok(PathsDemo)
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from("Paths Demo Rust Plugin"),
            signature: String::from("org.samcrow.xplm.examples.paths"),
            description: String::from("Demonstrates paths functions"),
        }
    }
}

xplane_plugin!(PathsDemo);
