//local shortcuts

//third-party shortcuts
use bevy_fn_plugin::bevy_plugin;
use bevy_girk_backend_public::*;
use bevy_kot::ecs::*;

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
#[derive(Debug)]
pub(crate) struct LobbyDisplay
{
    current    : Option<LobbyData>,
    lobby_type : Option<LobbyType>,
}

impl LobbyDisplay
{
    pub(crate) fn set(&mut self, new_lobby: LobbyData, lobby_type: LobbyType)
    {
        self.current    = Some(new_lobby);
        self.lobby_type = Some(lobby_type);
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

    pub(crate) fn get(&self) -> Option<&LobbyData>
    {
        self.current.as_ref()
    }

    pub(crate) fn _lobby_type(&self) -> Option<&LobbyType>
    {
        self.lobby_type.as_ref()
    }

    pub(crate) fn is_set(&self) -> bool
    {
        self.current.is_some()
    }

    pub(crate) fn _is_local(&self) -> bool
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
        .insert_resource(ReactRes::new(LobbyDisplay::default()))
        ;
}

//-------------------------------------------------------------------------------------------------------------------