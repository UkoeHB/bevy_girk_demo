use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use bevy_girk_client_fw::ClientAppState;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

fn try_spawn_reconnecting_overlay(mut c: Commands, mut s: SceneBuilder, starter: ReactRes<ClientStarter>)
{
    if !starter.has_starter() {
        return;
    }

    c.ui_root()
        .spawn_scene(("ui.user", "reconnecting_overlay"), &mut s, |h| {
            h.insert(StateScoped(ClientAppState::Client));

            h.update_on(
                resource_mutation::<ClientStarter>(),
                |id: TargetId, mut c: Commands, starter: ReactRes<ClientStarter>| {
                    if starter.has_starter() {
                        return DONE;
                    }
                    c.get_entity(*id)?.despawn();
                    DONE
                },
            );
        });
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct UiReconnectingPlugin;

impl Plugin for UiReconnectingPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(ClientAppState::Client), try_spawn_reconnecting_overlay);
    }
}

//-------------------------------------------------------------------------------------------------------------------
