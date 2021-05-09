extern crate xplm;

use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{ArrayRead, DataRead, ReadOnly, ReadWrite, StringRead};
use xplm::plugin::{Plugin, PluginInfo};
use xplm::{debugln, xplane_plugin};

struct DataRefPlugin {
    has_joystick: DataRef<bool, ReadOnly>,
    earth_mu: DataRef<f32, ReadOnly>,
    date: DataRef<i32, ReadWrite>,
    sim_build_string: DataRef<[u8], ReadOnly>,
    latitude: DataRef<f64, ReadOnly>,
    joystick_axis_values: DataRef<[f32], ReadOnly>,
    battery_on: DataRef<[i32], ReadWrite>,
}

impl DataRefPlugin {
    fn test_datarefs(&mut self) {
        debugln!("Has joystick: {}", self.has_joystick.get());
        debugln!("Earth mu: {}", self.earth_mu.get());
        debugln!("Date: {}", self.date.get());
        debugln!(
            "Simulator build: {}",
            self.sim_build_string
                .get_as_string()
                .unwrap_or(String::from("Unknown"))
        );
        debugln!("Latitude: {}", self.latitude.get());
        debugln!(
            "Joystick axis values: {:?}",
            self.joystick_axis_values.as_vec()
        );
        debugln!("Battery on: {:?}", self.battery_on.as_vec());
    }
}

impl Plugin for DataRefPlugin {
    type Error = FindError;
    fn start() -> Result<Self, Self::Error> {
        let plugin = DataRefPlugin {
            has_joystick: DataRef::find("sim/joystick/has_joystick")?,
            earth_mu: DataRef::find("sim/physics/earth_mu")?,
            date: DataRef::find("sim/time/local_date_days")?.writeable()?,
            sim_build_string: DataRef::find("sim/version/sim_build_string")?,
            latitude: DataRef::find("sim/flightmodel/position/latitude")?,
            joystick_axis_values: DataRef::find("sim/joystick/joystick_axis_values")?,
            battery_on: DataRef::find("sim/cockpit2/electrical/battery_on")?.writeable()?,
        };
        Ok(plugin)
    }

    fn enable(&mut self) -> Result<(), Self::Error> {
        self.test_datarefs();
        Ok(())
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from("Dataref Test"),
            signature: String::from("org.samcrow.xplm.examples.dataref"),
            description: String::from("Tests the DataRef features of xplm"),
        }
    }
}

xplane_plugin!(DataRefPlugin);
