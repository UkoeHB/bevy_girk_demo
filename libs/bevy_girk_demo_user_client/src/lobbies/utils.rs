//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_backend_public::*;
use bevy_kot::prelude::*;

//standard shortcuts

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn rerequest_latest_lobby_page(
    rcommands           : &mut ReactCommands,
    client              : &HostUserClient,
    target_entity       : Entity,
    last_lobby_page_req : &LobbyPageRequest,
){
    let Ok(new_req) = client.request(UserToHostRequest::LobbySearch(last_lobby_page_req.get().clone())) else { return; };
    rcommands.insert(target_entity, PendingRequest::new(new_req));
}

//-------------------------------------------------------------------------------------------------------------------
