//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_kot::ecs::{*, syscall};
use bevy_kot::ui::*;
use bevy_kot::ui::builtin::*;
use bevy_lunex::prelude::*;

//standard shortcuts


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



//-------------------------------------------------------------------------------------------------------------------
