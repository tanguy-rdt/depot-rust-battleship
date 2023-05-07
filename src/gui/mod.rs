mod cli_ui;
mod desktop_ui;

#[cfg(feature = "cli")]
pub use cli_ui::{update_ui, ask_ship_position, ask_target};

#[cfg(feature = "desktop")]
pub use desktop_ui::{update_ui, ask_ship_position, ask_target};

#[cfg(not(any(feature = "cli", feature = "desktop")))]
compile_error!("Please enable one of the UI features: cli, desktop");