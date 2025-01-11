use bevy::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn setup_lobby_tag_entities(mut commands: Commands)
{
    commands.spawn(JoinLobby);
    commands.spawn(LobbySearch);
    commands.spawn(MakeLobby);
    commands.spawn(LeaveLobby);
    commands.spawn(LaunchLobby);
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Component, Debug)]
pub(crate) struct JoinLobby;

#[derive(Component, Debug)]
pub(crate) struct LobbySearch;

#[derive(Component, Debug)]
pub(crate) struct MakeLobby;

#[derive(Component, Debug)]
pub(crate) struct LeaveLobby;

#[derive(Component, Debug)]
pub(crate) struct LaunchLobby;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) struct LobbiesPlugin;

impl Plugin for LobbiesPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(AckRequestPlugin)
            .add_plugins(LobbyDisplayPlugin)
            .add_plugins(LobbyPagePlugin)
            .add_plugins(LobbyListPlugin)
            .add_plugins(LobbyJoinWindowPlugin)
            .add_plugins(LobbyMakeWindowPlugin)
            .add_systems(PreStartup, setup_lobby_tag_entities);
    }
}

//-------------------------------------------------------------------------------------------------------------------
