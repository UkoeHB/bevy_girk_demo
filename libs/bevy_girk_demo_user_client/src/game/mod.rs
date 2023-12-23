//module tree
mod config_temp;
mod handle_client_instance_reports;
mod plugin;
mod tags;

//API exports
pub(crate) use crate::game::config_temp::*;
pub(crate) use crate::game::handle_client_instance_reports::*;
pub(crate) use crate::game::plugin::*;
pub(crate) use crate::game::tags::*;
