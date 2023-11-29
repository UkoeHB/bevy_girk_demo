//local shortcuts
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy_fn_plugin::bevy_plugin;
use bevy_girk_backend_public::*;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum LobbyType
{
    /// On the local machine (single-player).
    Local,
    /// On a host server.
    Hosted,
}

//-------------------------------------------------------------------------------------------------------------------

/// Caches the currently-displayed lobby that the user is a member of.
#[derive(ReactResource, Debug)]
pub(crate) struct LobbyDisplay
{
    current    : Option<ClickLobbyContents>,
    lobby_type : Option<LobbyType>,
}

impl LobbyDisplay
{
    pub(crate) fn set(&mut self, contents: ClickLobbyContents, lobby_type: LobbyType)
    {
        self.current    = Some(contents);
        self.lobby_type = Some(lobby_type);
    }

    /// Returns `Err` if lobby contents cannot be extracted from the lobby data.
    pub(crate) fn try_set(&mut self, new_lobby: LobbyData, lobby_type: LobbyType) -> Result<(), ()>
    {
        self.current    = Some(ClickLobbyContents::try_from(new_lobby)?);
        self.lobby_type = Some(lobby_type);

        Ok(())
    }

    pub(crate) fn clear(&mut self)
    {
        self.current    = None;
        self.lobby_type = None;
    }

    pub(crate) fn lobby_id(&self) -> Option<u64>
    {
        match &self.current
        {
            Some(data) => Some(data.id),
            None       => None,
        }
    }

    pub(crate) fn get(&self) -> Option<&ClickLobbyContents>
    {
        self.current.as_ref()
    }

    pub(crate) fn lobby_type(&self) -> Option<LobbyType>
    {
        self.lobby_type
    }

    pub(crate) fn is_set(&self) -> bool
    {
        self.current.is_some()
    }

    pub(crate) fn is_local(&self) -> bool
    {
        match self.lobby_type
        {
            Some(LobbyType::Local) => true,
            _                      => false,
        }
    }

    pub(crate) fn is_hosted(&self) -> bool
    {
        match self.lobby_type
        {
            Some(LobbyType::Hosted) => true,
            _                      => false,
        }
    }
}

impl Default for LobbyDisplay { fn default() -> Self { Self{ current: None, lobby_type: None } } }

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn LobbyDisplayPlugin(app: &mut App)
{
    app
        .insert_react_resource(LobbyDisplay::default())
        ;
}

//-------------------------------------------------------------------------------------------------------------------
