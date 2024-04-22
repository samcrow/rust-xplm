use xplm_sys;

#[derive(Debug)]
pub struct VersionInfo {
    pub xplane_version: i32,
    pub xplm_version: i32,
    pub host_id: i32,
}

impl VersionInfo {
    pub fn get() -> Self {
        let mut xplane_version: i32 = -1;
        let mut xplm_version: i32 = -1;
        let mut host_id: i32 = -1;

        unsafe {
            xplm_sys::XPLMGetVersions(&mut xplane_version, &mut xplm_version, &mut host_id);
        }

        VersionInfo {
            xplane_version,
            xplm_version,
            host_id,
        }
    }
}
