use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_backend_public::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) fn rerequest_latest_lobby_page(
    c: &mut Commands,
    client: &HostUserClient,
    target_entity: Entity,
    last_lobby_page_req: &LobbyPageRequest,
)
{
    let new_req = client.request(UserToHostRequest::LobbySearch(last_lobby_page_req.get().clone()));
    c.react()
        .insert(target_entity, PendingRequest::new(new_req));
}

//-------------------------------------------------------------------------------------------------------------------
