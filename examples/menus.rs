//!
//! This plugin creates a submenu under the Plugins menu. The submenu has one checkable item
//! and one action item.
//!

extern crate xplm;

use xplm::menu::{ActionItem, CheckHandler, CheckItem, Menu, MenuClickHandler};
use xplm::plugin::{Plugin, PluginInfo};
use xplm::{debugln, xplane_plugin};

struct MenuPlugin {
    _plugins_submenu: Menu,
}

impl Plugin for MenuPlugin {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        let plugins_submenu = Menu::new("Menu Test Plugin").unwrap();
        plugins_submenu.add_child(CheckItem::new("Checkable 1", false, CheckHandler1).unwrap());
        plugins_submenu.add_child(ActionItem::new("Action 1", ActionHandler1).unwrap());
        plugins_submenu.add_to_plugins_menu();

        // The menu needs to be part of the plugin struct, or it will immediately get dropped and
        // will not appear
        Ok(MenuPlugin {
            _plugins_submenu: plugins_submenu,
        })
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from("Rust Menu Plugin"),
            signature: String::from("org.samcrow.xplm.examples.menu"),
            description: String::from("A plugin written in Rust that creates menus and menu items"),
        }
    }
}

xplane_plugin!(MenuPlugin);

struct CheckHandler1;

impl CheckHandler for CheckHandler1 {
    fn item_checked(&mut self, _item: &CheckItem, checked: bool) {
        debugln!("Checkable 1 checked = {}", checked);
    }
}

struct ActionHandler1;

impl MenuClickHandler for ActionHandler1 {
    fn item_clicked(&mut self, _item: &ActionItem) {
        debugln!("Action 1 selected");
    }
}
