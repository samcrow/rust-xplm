
use super::feature;

/// Enables native paths
pub fn path_init() {
    // Feature specified to exist in SDK 2.1
    let native_path_feature =
        feature::find_feature("XPLM_USE_NATIVE_PATHS").expect("No native paths feature");
    native_path_feature.set_enabled(true);
}
