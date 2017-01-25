
/// Information about a plugin
pub struct PluginInfo {
    /// The plugin name
    pub name: String,
    /// The plugin's signature, in reverse DNS format
    pub signature: String,
    /// A description of the plugin
    pub description: String,
}

/// The trait that all plugins should implement
pub trait Plugin: Sized {
    /// The error type that a plugin may encounter when starting up
    type StartErr: ::std::error::Error;

    /// Called when X-Plane loads this plugin
    /// On success, returns a plugin object
    fn start() -> Result<Self, Self::StartErr>;
    /// Called when the plugin is enabled
    fn enable(&mut self);
    /// Called when the plugin is disabled
    fn disable(&mut self);

    /// Returns information on this plugin
    fn info(&self) -> PluginInfo;

    /// Called when the plugin is stopped
    ///
    /// The plugin will be dropped after this function is called.
    fn stop(&mut self);
}
