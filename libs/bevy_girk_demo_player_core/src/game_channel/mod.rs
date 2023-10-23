//module tree
mod handle_game_incoming;
mod handle_game_incoming_impl;

//API exports
pub(crate) use crate::game_channel::handle_game_incoming::*;
pub(crate) use crate::game_channel::handle_game_incoming_impl::*;
