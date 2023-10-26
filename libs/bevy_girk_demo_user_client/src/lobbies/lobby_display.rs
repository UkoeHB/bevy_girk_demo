//local shortcuts

//third-party shortcuts
use bevy_fn_plugin::bevy_plugin;
use bevy_girk_backend_public::*;
use bevy_kot::ecs::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

/// Caches the currently-displayed lobby that the user is a member of.
#[derive(Debug)]
pub(crate) struct LobbyDisplay
{
    current: Option<LobbyData>,
}

impl LobbyDisplay
{
    pub(crate) fn set(&mut self, new_lobby: LobbyData)
    {
        self.current = Some(new_lobby);
    }

    pub(crate) fn clear(&mut self)
    {
        self.current = None;
    }

    pub(crate) fn lobby_id(&self) -> Option<u64>
    {
        match &self.current
        {
            Some(data) => Some(data.id),
            None       => None,
        }
    }

    pub(crate) fn _get(&self) -> Option<&LobbyData>
    {
        self.current.as_ref()
    }

    pub(crate) fn is_set(&self) -> bool
    {
        self.current.is_some()
    }
}

impl Default for LobbyDisplay { fn default() -> Self { Self{ current: None } } }

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn LobbyDisplayPlugin(app: &mut App)
{
    app
        .insert_resource(ReactRes::new(LobbyDisplay::default()))
        ;
}

//-------------------------------------------------------------------------------------------------------------------
