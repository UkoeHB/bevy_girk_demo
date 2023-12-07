//local shortcuts

//third-party shortcuts
use bevy::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Client core mode
#[derive(States, Debug, Default, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ClientMode
{
    #[default]
    Init,
    Prep,
    Play,
    GameOver
}

//-------------------------------------------------------------------------------------------------------------------
