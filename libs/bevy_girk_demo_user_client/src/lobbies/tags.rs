use bevy::prelude::*;

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

pub(crate) fn setup_lobby_tag_entities(mut commands: Commands)
{
    commands.spawn(JoinLobby);
    commands.spawn(LobbySearch);
    commands.spawn(MakeLobby);
    commands.spawn(LeaveLobby);
    commands.spawn(LaunchLobby);
}

//-------------------------------------------------------------------------------------------------------------------
