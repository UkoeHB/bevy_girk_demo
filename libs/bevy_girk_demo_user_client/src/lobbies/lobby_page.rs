//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_girk_backend_public::*;
use bevy_kot::ecs::*;

//standard shortcuts
use std::vec::Vec;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn initialize_lobby_page(
    mut rcommands  : ReactCommands,
    lobbies_config : Res<LobbiesConfig>,
    query          : Query<Entity, With<LobbySearch>>,
){
    let entity = query.single();

    rcommands
        .commands()
        .entity(entity)
        .insert(
            (
                LobbyPageRequest::new(LobbySearchRequest::Page{
                    youngest_lobby_id : 0u64,
                    num_lobbies       : lobbies_config.lobby_page_size,
                }),
            )
        );
    rcommands.insert(entity, LobbyPage::default());
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Caches the currently-displayed lobby page.
#[derive(Debug)]
pub(crate) struct LobbyPage
{
    current: Vec<LobbyData>,
}

impl LobbyPage
{
    pub(crate) fn set(&mut self, new_page: Vec<LobbyData>)
    {
        self.current = new_page;
    }

    pub(crate) fn _clear(&mut self)
    {
        self.current = vec![];
    }

    pub(crate) fn _get(&self) -> &Vec<LobbyData>
    {
        &self.current
    }
}

impl Default for LobbyPage { fn default() -> Self { Self{ current: Vec::default() } } }

//-------------------------------------------------------------------------------------------------------------------

/// Tracks the last lobby search request sent to the host server.
///
/// On startup this is initialized with the top-most lobby page.
#[derive(Component, Debug)]
pub(crate) struct LobbyPageRequest
{
    last: LobbySearchRequest,
}

impl LobbyPageRequest
{
    pub(crate) fn new(request: LobbySearchRequest) -> Self
    {
        Self{ last: request }
    }

    pub(crate) fn _set(&mut self, request: LobbySearchRequest)
    {
        self.last = request;
    }

    pub(crate) fn get(&self) -> &LobbySearchRequest
    {
        &self.last
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn LobbyPagePlugin(app: &mut App)
{
    app
        .add_systems(Startup, initialize_lobby_page)
        ;
}

//-------------------------------------------------------------------------------------------------------------------
