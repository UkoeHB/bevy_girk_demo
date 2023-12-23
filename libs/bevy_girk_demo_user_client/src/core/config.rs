//local shortcuts

//third-party shortcuts
use bevy::prelude::*;
use bevy_girk_user_client_utils::*;
use serde::{Deserialize, Serialize};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) const CLIENT_BACKDROP : (&'static str, Vec2) = ("userclient_backdrop.png", Vec2::new(640.0, 480.0));
pub(crate) const LOBBY_BACKDROP  : (&'static str, Vec2) = ("lobby_backdrop.png", Vec2::new(640.0, 480.0));
pub(crate) const POPUP_BACKDROP  : (&'static str, Vec2) = ("userclient_popup_backdrop.png", Vec2::new(640.0, 480.0));
pub(crate) const MISC_BUTTON     : (&'static str, Vec2) = ("button.png", Vec2::new(250.0, 142.0));
pub(crate) const MENU_BAR_BUTTON : (&'static str, Vec2) = MISC_BUTTON;
pub(crate) const _OUTLINE         : (&'static str, Vec2) = ("outline.png", Vec2::new(236.0, 139.0));
pub(crate) const BOX             : (&'static str, Vec2) = ("box.png", Vec2::new(236.0, 139.0));
pub(crate) const _FILM            : (&'static str, Vec2) = ("film.png", Vec2::new(236.0, 139.0));
pub(crate) const _BASIC_FONT      : &'static str = "fonts/FiraSans-Bold.ttf";

//-------------------------------------------------------------------------------------------------------------------

pub(crate) const NUM_LOBBY_CONTENT_ENTRIES : usize = 8;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct TimerConfigs
{
    /// Timeout after which an ack request expires.
    pub ack_request_timeout_ms: u64,
    /// Amount of time the user client ack request should 'shave off' the ack request timeout for displaying to users.
    pub ack_request_timer_buffer_ms: u64,
    /// Refresh interval for the lobby list.
    pub lobby_list_refresh_ms: u64,
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Clone, Debug)]
pub struct ClientLaunchConfigsGeneric<S: HandleReqs>
{
    pub local: LocalPlayerLauncherConfig<S>,
    pub multiplayer: MultiPlayerLauncherConfig<S>,
}

//-------------------------------------------------------------------------------------------------------------------

//todo: update this to use bevy's task pool?
pub type ClientLaunchConfigs = ClientLaunchConfigsGeneric<enfync::builtin::Handle>;

//-------------------------------------------------------------------------------------------------------------------
