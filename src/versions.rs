use xplm_sys;

pub struct VersionInfo {
    pub xplane_version: i32,
    pub xplm_version: i32,
    pub host_id: i32,
}

impl VersionInfo {
    pub fn get() -> Self {
        let xplane_version: *mut i32 = std::ptr::null_mut();
        let xplm_version: *mut i32 = std::ptr::null_mut();
        let host_id: *mut i32 = std::ptr::null_mut();

        unsafe {
            xplm_sys::XPLMGetVersions(xplane_version, xplm_version, host_id);
            return VersionInfo {
                xplane_version: *xplane_version.as_ref().unwrap_or(&-1),
                xplm_version: *xplm_version.as_ref().unwrap_or(&-1),
                host_id: *host_id.as_ref().unwrap_or(&-1),
            };
        }
    }
}
