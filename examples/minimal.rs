extern crate xplm;

use xplm::plugin::{Plugin, PluginInfo};
use xplm::{debugln, xplane_plugin};

struct MinimalPlugin;

impl Plugin for MinimalPlugin {
    type Error = MinimalPluginError;
    fn start() -> Result<Self, Self::Error> {
        // The following message should be visible in the developer console and the Log.txt file
        debugln!("Hello, World, from the Minimal Rust Plugin");
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

#[derive(Debug)]
enum MinimalPluginError {}

impl std::fmt::Display for MinimalPluginError {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {}
    }
}

impl std::error::Error for MinimalPluginError {
    fn description(&self) -> &str {
        match *self {}
    }
}

xplane_plugin!(MinimalPlugin);
