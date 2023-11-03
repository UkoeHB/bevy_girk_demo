//local shortcuts
use crate::*;
use bevy_girk_demo_wiring::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_fn_plugin::*;
use bevy_girk_backend_public::*;
use bevy_girk_utils::*;
use bevy_kot::ecs::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Cached state of the make lobby window.
///
/// This is a reactive resource.
#[derive(Debug)]
struct MakeLobbyWindow
{
    member_type : ClickLobbyMemberType,
    password    : String,
}

impl Default for MakeLobbyWindow
{
    fn default() -> Self
    {
        Self{ member_type: ClickLobbyMemberType::Player, password: String::default() }
    }
}

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

/// This is a reactive data event used to activate the window.
#[derive(Debug)]
pub(crate) struct ActivateMakeLobbyWindow;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn add_make_lobby_window(ctx: &mut UiBuilderCtx)
{

}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn UiMakeLobbyWindowPlugin(app: &mut App)
{

}

//-------------------------------------------------------------------------------------------------------------------
