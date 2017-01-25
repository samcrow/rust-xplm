
#[macro_use(xplane_plugin)]
extern crate xplm;
use xplm::plugin::{Plugin, PluginInfo};

struct TestPlugin;

impl Plugin for TestPlugin {
    type StartErr = TestPluginError;
    fn start() -> Result<Self, Self::StartErr> {
        Ok(TestPlugin)
    }
    fn enable(&mut self) {}
    fn disable(&mut self) {}

    fn stop(&mut self) {}
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "Test Plugin".into(),
            signature: "org.samcrow.rustplugin.test".into(),
            description: "A plugin written in Rust".into(),
        }
    }
}

#[derive(Debug)]
struct TestPluginError;

impl ::std::fmt::Display for TestPluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TestPluginError")
    }
}

impl ::std::error::Error for TestPluginError {
    fn description(&self) -> &str {
        "TestPluginError"
    }
}

xplane_plugin!(TestPlugin);

fn main() {}
