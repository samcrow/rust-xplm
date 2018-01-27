
#[macro_use(xplane_plugin)]
extern crate xplm;
use xplm::plugin::{Plugin, PluginInfo};

struct TestPlugin;

impl Plugin for TestPlugin {
    type Error = TestPluginError;
    fn start() -> Result<Self, Self::Error> {
        Ok(TestPlugin)
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "Test Plugin".into(),
            signature: "org.samcrow.rustplugin.test".into(),
            description: "A plugin written in Rust".into(),
        }
    }
}

#[derive(Debug)]
enum TestPluginError {}

impl ::std::fmt::Display for TestPluginError {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {}
    }
}

impl ::std::error::Error for TestPluginError {
    fn description(&self) -> &str {
        match *self {}
    }
}

xplane_plugin!(TestPlugin);
