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

fn send_make_lobby_request(
    In((
        member_type,
        pwd,
        config,
    ))            : In<(ClickLobbyMemberType, String, ClickLobbyConfig)>,
    mut rcommands : ReactCommands,
    client        : Res<HostUserClient>,
    make_lobby    : Query<Entity, With<MakeLobby>>,
){
    // request to make a lobby
    // - note: do not log the password
    tracing::trace!(?member_type, ?config, "requesting to make lobby");

    let Ok(new_req) = client.request(
            UserToHostRequest::MakeLobby{
                    mcolor: member_type.into(),
                    pwd,
                    data: ser_msg(&config)
                }
        )
    else { return; };

    // save request
    let target_entity = make_lobby.single();
    rcommands.insert(target_entity, PendingRequest::new(new_req));
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------



//-------------------------------------------------------------------------------------------------------------------
