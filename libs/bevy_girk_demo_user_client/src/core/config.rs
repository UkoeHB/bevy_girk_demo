//local shortcuts

//third-party shortcuts
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) const MISC_BUTTON          : (&'static str, Vec2) = ("button.png", Vec2::new(250.0, 142.0));
pub(crate) const MENU_BAR_BUTTON      : (&'static str, Vec2) = MISC_BUTTON;
pub(crate) const OUTLINE              : (&'static str, Vec2) = ("outline.png", Vec2::new(236.0, 139.0));
pub(crate) const BOX                  : (&'static str, Vec2) = ("box.png", Vec2::new(236.0, 139.0));
pub(crate) const FILM                 : (&'static str, Vec2) = ("film.png", Vec2::new(236.0, 139.0));
pub(crate) const BASIC_FONT           : &'static str = "fonts/FiraSans-Bold.ttf";

//-------------------------------------------------------------------------------------------------------------------

pub(crate) const NUM_LOBBY_CONTENT_ENTRIES : usize = 8;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct TimerConfigs
{
    /// Timeout after which an ack request expires.
    pub ack_request_timeout_ms: u64,
    /// Refresh interval for the lobby list.
    pub lobby_list_refresh_ms: u64,
}

//-------------------------------------------------------------------------------------------------------------------
