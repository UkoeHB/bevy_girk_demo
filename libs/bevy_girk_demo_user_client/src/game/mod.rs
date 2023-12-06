//module tree
mod config_temp;
mod game_launchers;
mod game_monitor;
mod plugin;

#[cfg(not(target_family = "wasm"))]
mod native;
#[cfg(target_family = "wasm")]
mod wasm;

//API exports
pub(crate) use crate::game::config_temp::*;
pub(crate) use crate::game::game_launchers::*;
pub(crate) use crate::game::game_monitor::*;
pub(crate) use crate::game::plugin::*;

#[cfg(not(target_family = "wasm"))]
pub(crate) use crate::game::native::*;
#[cfg(target_family = "wasm")]
pub(crate) use crate::game::wasm::*;
