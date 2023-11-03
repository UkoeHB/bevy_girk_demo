//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_kot::ecs::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Cached state of the join lobby window.
///
/// This is a reactive resource.
#[derive(Debug)]
struct JoinLobbyWindow
{
    lobby_id: u64,
}

impl Default for JoinLobbyWindow
{
    fn default() -> Self
    {
        Self{ lobby_id: u64::MAX }
    }
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn send_join_lobby_request(
    In((
        lobby_id,
        member_type,
        pwd,
    ))            : In<(u64, ClickLobbyMemberType, String)>,
    mut rcommands : ReactCommands,
    client        : Res<HostUserClient>,
    join_lobby    : Query<Entity, With<JoinLobby>>,
){
    // request to join the specified lobby
    // - note: do not log the password
    tracing::trace!(lobby_id, ?member_type, "requesting to join lobby");

    let Ok(new_req) = client.request(
            UserToHostRequest::JoinLobby{ id: lobby_id, mcolor: member_type.into(), pwd }
        )
    else { return; };

    // save request
    let target_entity = join_lobby.single();
    rcommands.insert(target_entity, PendingRequest::new(new_req));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// This is a reactive data event used to activate the window.
#[derive(Debug)]
pub(crate) struct ActivateJoinLobbyWindow
{
    /// Index in the lobby list of the lobby corresponding to this lobby window.
    ///
    /// Only valid in the tick where it is set.
    pub(crate) lobby_list_index: usize,
}

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_join_lobby_window(ctx: &mut UiBuilderCtx)
{
    
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiJoinLobbyWindowPlugin(app: &mut App)
{

}

//-------------------------------------------------------------------------------------------------------------------
