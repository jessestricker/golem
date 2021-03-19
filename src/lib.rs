use const_format::formatcp;

/// The name of the Cargo package.
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
/// The current version of the Cargo package.
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The name of the control application.
pub const APP_NAME_CONTROL: &str = formatcp!("{}ctl", PKG_NAME);
/// The name of the daemon application.
pub const APP_NAME_DAEMON: &str = formatcp!("{}d", PKG_NAME);

/// The title (name and version) of the control application.
pub const APP_TITLE_CONTROL: &str = formatcp!("{} v{}", APP_NAME_CONTROL, PKG_VERSION);
/// The title (name and version) of the daemon application.
pub const APP_TITLE_DAEMON: &str = formatcp!("{} v{}", APP_NAME_DAEMON, PKG_VERSION);
