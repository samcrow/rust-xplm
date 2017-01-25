
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
        type PluginType = $plugin_type;
        type PluginPtr = *mut PluginType;
        // The plugin
        static mut PLUGIN: PluginPtr = 0 as PluginPtr;

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginStart(
            outName: *mut ::std::os::raw::c_char,
            outSig: *mut ::std::os::raw::c_char,
            outDescription: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int
        {
            // Create the plugin, temporarily, on the stack
            let plugin_option = PluginType::start();

            match plugin_option {
                Ok(plugin) => {
                    // Allocate storage
                    PLUGIN = Box::into_raw(Box::new(plugin));

                    let info = (*PLUGIN).info();
                    ::xplm::internal::copy_to_c_buffer(info.name, outName);
                    ::xplm::internal::copy_to_c_buffer(info.signature, outSig);
                    ::xplm::internal::copy_to_c_buffer(info.description, outDescription);
                    // Success
                    1
                },
                Err(e) => {
                    let message = format!("Plugin initialization failed: {}\n", e);
                    ::xplm::debug(&message);
                    // Return failure
                    0
                },
            }
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginStop() {
            (*PLUGIN).stop();
            // Free plugin
            let plugin_box = Box::from_raw(PLUGIN);
            PLUGIN = ::std::ptr::null_mut();
            drop(plugin_box);
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginEnable() {
            (*PLUGIN).enable();
        }

        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginDisable() {
            (*PLUGIN).disable();
        }

        #[allow(non_snake_case)]
        #[allow(unused_variables)]
        #[no_mangle]
        pub unsafe extern "C" fn XPluginReceiveMessage(
            inFrom: ::std::os::raw::c_int,
            inMessage: ::std::os::raw::c_int,
            inParam: *mut ::std::os::raw::c_void)
        {
            // Nothing
        }

    }
}
