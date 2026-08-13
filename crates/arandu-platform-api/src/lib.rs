use arandu_model::ApplicationNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformError {
    pub message: String,
}

impl PlatformError {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub trait PlatformAdapter {
    /// Lists applications currently open on the host platform.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformError`] when the platform cannot enumerate open applications.
    fn list_open_applications(&self) -> Result<Vec<ApplicationNode>, PlatformError>;

    /// Returns the application that currently owns focus, when one exists.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformError`] when the platform cannot determine the focused application.
    fn focused_application(&self) -> Result<Option<ApplicationNode>, PlatformError>;

    /// Lists applications known to be installed on the host platform.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformError`] when the platform cannot enumerate installed applications.
    fn installed_applications(&self) -> Result<Vec<ApplicationNode>, PlatformError>;

    /// Brings an already running application to the foreground.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformError`] when the platform cannot activate the requested application.
    fn activate_application(&self, app: &ApplicationNode) -> Result<(), PlatformError>;

    /// Launches the requested application.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformError`] when the platform cannot launch the requested application.
    fn launch_application(&self, app: &ApplicationNode) -> Result<(), PlatformError>;
}
