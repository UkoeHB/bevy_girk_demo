use bevy::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

/// Client core mode
#[derive(SubStates, Debug, Default, Eq, PartialEq, Hash, Copy, Clone)]
#[source(ClientAppState = ClientAppState::Game)]
pub enum ClientState
{
    #[default]
    Init,
    Prep,
    Play,
    GameOver,
}

//-------------------------------------------------------------------------------------------------------------------
