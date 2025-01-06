use bevy::prelude::*;
use bevy_girk_game_fw::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Client core sets.
///
/// These sets are modal in schedule `Update`.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ClientSet
{
    /// Runs initialization logic.
    Init,
    /// Runs in game mode 'prep' (but not when initializing).
    Prep,
    /// Runs in game mode 'play' (but not when initializing).
    Play,
    /// Runs in game mode 'game over' (but not when initializing).
    GameOver,
}

//-------------------------------------------------------------------------------------------------------------------

/// Client core logic sets.
///
/// These sets are ordinal in schedule `Update`.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ClientLogicSet
{
    Admin,
    Update,
}

//-------------------------------------------------------------------------------------------------------------------

/// Configures root-level system sets.
pub(crate) struct ClientSetsPlugin;

impl Plugin for ClientSetsPlugin
{
    fn build(&self, app: &mut App)
    {
        app.configure_sets(
            Update,
            ClientSet::Init
                .run_if(client_is_initializing)
                .run_if(in_state(ClientState::Init)),
        )
        .configure_sets(
            Update,
            ClientSet::Prep
                .run_if(in_state(ClientFwState::Game))
                .run_if(in_state(ClientState::Prep)),
        )
        .configure_sets(
            Update,
            ClientSet::Play
                .run_if(in_state(ClientFwState::Game))
                .run_if(in_state(ClientState::Play)),
        )
        .configure_sets(
            Update,
            ClientSet::GameOver
                .run_if(in_state(ClientFwState::End))
                .run_if(in_state(ClientState::GameOver)),
        )
        .configure_sets(
            Update,
            (ClientLogicSet::Admin, ClientLogicSet::Update)
                .chain()
                .run_if(in_state(ClientInstanceState::Game)),
        );
    }
}

//-------------------------------------------------------------------------------------------------------------------
