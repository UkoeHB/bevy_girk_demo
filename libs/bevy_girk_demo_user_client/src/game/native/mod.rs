//module tree
mod local_player;
mod multiplayer;

//API exports
pub(crate) use crate::game::native::local_player::*;
pub(crate) use crate::game::native::multiplayer::*;
