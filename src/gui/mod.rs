pub mod ui;
mod cli_ui;
mod desktop_ui;

pub use cli_ui::CliUi;
pub use desktop_ui::DesktopUi;

#[cfg(feature = "cli")]
pub type Gui = CliUi;

#[cfg(feature = "desktop")]
pub type Gui = DesktopUi;

#[cfg(not(any(feature = "cli", feature = "desktop")))]
compile_error!("Please enable one of the UI features: cli, desktop");