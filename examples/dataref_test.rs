
#[macro_use(xplane_plugin)]
extern crate xplm;
use xplm::plugin::{Plugin, PluginInfo};

use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{ReadOnly, ReadWrite, DataRead, ArrayRead, StringRead};

struct DatarefPlugin {
    has_joystick: DataRef<bool, ReadOnly>,
    earth_mu: DataRef<f32, ReadOnly>,
    date: DataRef<i32, ReadWrite>,
    sim_build_string: DataRef<[u8], ReadOnly>,
    latitude: DataRef<f64, ReadOnly>,
    joystick_axis_values: DataRef<[f32], ReadOnly>,
    battery_on: DataRef<[i32], ReadWrite>,
}

impl DatarefPlugin {
    fn test_datarefs(&mut self) {
        xplm::debug(format!("has joystick: {}\n", self.has_joystick.get()));
        xplm::debug(format!("earth mu: {}\n", self.earth_mu.get()));
        xplm::debug(format!("date: {}\n", self.date.get()));
        xplm::debug(format!(
            "simulator build: {}\n",
            self.sim_build_string.get_as_string().unwrap_or(
                "unknown".into(),
            )
        ));
        xplm::debug(format!("latitude: {}\n", self.latitude.get()));
        xplm::debug(format!(
            "joystick axis values: {:?}\n",
            self.joystick_axis_values.as_vec()
        ));
        xplm::debug(format!("battery on: {:?}\n", self.battery_on.as_vec()));
    }
}

impl Plugin for DatarefPlugin {
    type Error = FindError;
    fn start() -> Result<Self, Self::Error> {
        let mut plugin = DatarefPlugin {
            has_joystick: DataRef::find("sim/joystick/has_joystick")?,
            earth_mu: DataRef::find("sim/physics/earth_mu")?,
            date: DataRef::find("sim/time/local_date_days")?.writeable()?,
            sim_build_string: DataRef::find("sim/version/sim_build_string")?,
            latitude: DataRef::find("sim/flightmodel/position/latitude")?,
            joystick_axis_values: DataRef::find("sim/joystick/joystick_axis_values")?,
            battery_on: DataRef::find("sim/cockpit2/electrical/battery_on")?.writeable()?,
        };
        plugin.test_datarefs();
        Ok(plugin)
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "Dataref test".into(),
            signature: "org.samcrow.xplm.examples.datareftest".into(),
            description: "Tests the dataref features of xplm".into(),
        }
    }

    fn enable(&mut self) -> Result<(), Self::Error> {
        self.test_datarefs();
        Ok(())
    }
    fn disable(&mut self) {
        self.test_datarefs();
    }
}

xplane_plugin!(DatarefPlugin);

