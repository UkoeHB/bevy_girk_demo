use bevy::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

/// Client core mode
#[derive(States, Debug, Default, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ClientMode
{
    #[default]
    Init,
    Prep,
    Play,
    GameOver,
}

//-------------------------------------------------------------------------------------------------------------------
