/// Creates an X-Plane plugin
///
/// Provide the name of your plugin struct. The callbacks that X-Plane uses will be created.
///
/// Creating a plugin involves three steps:
///
/// 1. Create a struct for your plugin
/// 2. Implement Plugin for your plugin struct
/// 3. Place `xplane_plugin!(YourPluginStruct)` in a file, not in any function
///
#[macro_export]
macro_rules! xplane_plugin {
    ($plugin_type: ty) => {
        // The plugin
        static mut PLUGIN: ::xplm::plugin::internal::PluginData<$plugin_type> =
            ::xplm::plugin::internal::PluginData {
                plugin: 0 as *mut _,
                panicked: false,
            };

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginStart(
            name: *mut ::std::os::raw::c_char,
            signature: *mut ::std::os::raw::c_char,
            description: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int {
            ::xplm::plugin::internal::xplugin_start(&mut PLUGIN, name, signature, description)
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginStop() {
            ::xplm::plugin::internal::xplugin_stop(&mut PLUGIN)
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginEnable() -> ::std::os::raw::c_int {
            ::xplm::plugin::internal::xplugin_enable(&mut PLUGIN)
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginDisable() {
            ::xplm::plugin::internal::xplugin_disable(&mut PLUGIN)
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginReceiveMessage(
            from: ::std::os::raw::c_int,
            message: ::std::os::raw::c_int,
            param: *mut ::std::os::raw::c_void,
        ) {
            // Nothing
        }
    };
}
