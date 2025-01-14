use bevy::prelude::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn setup_lobby_tag_entities(mut c: Commands)
{
    spawn_request_entity(&mut c, JoinLobby);
    spawn_request_entity(&mut c, LobbySearch);
    spawn_request_entity(&mut c, MakeLobby);
    spawn_request_entity(&mut c, LeaveLobby);
    spawn_request_entity(&mut c, LaunchLobby);
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
            .add_plugins(JoinLobbyPlugin)
            .add_plugins(MakeLobbyPlugin)
            .add_systems(PreStartup, setup_lobby_tag_entities);
    }
}

//-------------------------------------------------------------------------------------------------------------------
