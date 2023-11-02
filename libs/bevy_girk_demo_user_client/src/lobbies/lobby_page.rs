//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy_fn_plugin::bevy_plugin;
use bevy_girk_backend_public::*;
use bevy_kot::ecs::*;

//standard shortcuts
use std::vec::Vec;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Caches the currently-displayed lobby page.
#[derive(Debug)]
pub(crate) struct LobbyPage
{
    current: Vec<ClickLobbyContents>,
    //todo: total number of lobbies on server
    //todo: index of youngest lobby in server's lobby list
}

impl LobbyPage
{
    pub(crate) fn try_set(&mut self, new_page: Vec<LobbyData>) -> Result<(), ()>
    {
        let mut temp = Vec::with_capacity(new_page.len());

        for lobby_data in new_page
        {
            temp.push(ClickLobbyContents::try_from(lobby_data)?);
        }

        self.current = temp;
        Ok(())
    }

    pub(crate) fn _clear(&mut self)
    {
        self.current = vec![];
    }

    pub(crate) fn get(&self) -> &Vec<ClickLobbyContents>
    {
        &self.current
    }

    pub(crate) fn len(&self) -> usize
    {
        self.current.len()
    }
}

impl Default for LobbyPage { fn default() -> Self { Self{ current: Vec::default() } } }

//-------------------------------------------------------------------------------------------------------------------

/// Tracks the last lobby search request sent to the host server.
///
/// On startup this is initialized with the top-most lobby page.
#[derive(Debug)]
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

    pub(crate) fn set(&mut self, request: LobbySearchRequest)
    {
        self.last = request;
    }

    pub(crate) fn get(&self) -> &LobbySearchRequest
    {
        &self.last
    }

    pub(crate) fn is_now(&self) -> bool
    {
        match self.last
        {
            LobbySearchRequest::LobbyId(_)                             => false,
            LobbySearchRequest::Page{ youngest_lobby_id, num_lobbies: _ } => youngest_lobby_id == u64::MAX,
        }
    }

    pub(crate) fn is_oldest(&self) -> bool
    {
        match self.last
        {
            LobbySearchRequest::LobbyId(_)                             => false,
            LobbySearchRequest::Page{ youngest_lobby_id: _, num_lobbies: _ } =>
            {
                //todo: need special request type to handle this
                tracing::error!("oldest page lobby search not yet implemented");
                true
            }
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn LobbyPagePlugin(app: &mut App)
{
    app
        .insert_resource(ReactRes::new(LobbyPage::default()))
        .insert_resource(ReactRes::new(LobbyPageRequest::new(LobbySearchRequest::Page{
            youngest_lobby_id : u64::MAX,
            num_lobbies       : LOBBY_LIST_SIZE as u16,
        })))
        ;
}

//-------------------------------------------------------------------------------------------------------------------
