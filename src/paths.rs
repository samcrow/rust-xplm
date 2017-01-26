use std::path::PathBuf;

#[cfg(all(target_os = "macos", not(feature = "xplm210")))]
use hfs_paths;

/// When SDK 2.1 is available, enables native paths
#[cfg(feature = "xplm210")]
pub fn path_init() {
    use super::feature;
    // Feature specified to exist in SDK 2.1
    let native_path_feature = feature::find_feature("XPLM_USE_NATIVE_PATHS").unwrap();
    native_path_feature.set_enabled(true);
}

// When SDK 2.1 is not available, does nothing
#[cfg(not(feature = "xplm210"))]
pub fn path_init() {

}

/// Converts an HFS path into a standard Unix-type path with / as a directory separator
///
/// This is needed for SDK versions before 2.1, where XPLM_USE_NATIVE_PATHS is not available.
#[cfg(all(target_os = "macos", not(feature = "xplm210")))]
pub fn convert_path(path: &str) -> Result<PathBuf, PathError> {
    hfs_paths::convert_path(path)
}

/// Converts a path into a PathBuf with no modifications
#[cfg(not(all(target_os = "macos", not(feature = "xplm210"))))]
pub fn convert_path(path: &str) -> Result<PathBuf, PathError> {
    Ok(PathBuf::from(path))
}


#[cfg(all(target_os = "macos", not(feature = "xplm210")))]
pub use hfs_paths::Error as PathError;


#[cfg(not(all(target_os = "macos", not(feature = "xplm210"))))]
quick_error! {
    /// Path conversion will always work on non-Mac OS platforms and with SDK 2.1
    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum PathError {
    }
}
