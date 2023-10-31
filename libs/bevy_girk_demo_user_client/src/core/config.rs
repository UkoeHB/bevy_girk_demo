//local shortcuts

//third-party shortcuts
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) const MISC_BUTTON          : &'static str = "button.png";
pub(crate) const MENU_BAR_BUTTON      : &'static str = "button.png";
pub(crate) const OUTLINE              : &'static str = "outline.png";
pub(crate) const BOX                  : &'static str = "box.png";
pub(crate) const MISC_FONT            : &'static str = "fonts/FiraSans-Bold.ttf";
pub(crate) const STATUS_FONT          : &'static str = "fonts/FiraSans-Bold.ttf";
pub(crate) const MENU_BAR_BUTTON_FONT : &'static str = "fonts/FiraSans-Bold.ttf";

pub(crate) const MISC_FONT_COLOR            : Color = Color::WHITE;
pub(crate) const STATUS_FONT_COLOR          : Color = Color::WHITE;
pub(crate) const MENU_BAR_BUTTON_FONT_COLOR : Color = Color::WHITE;
pub(crate) const DISABLED_BUTTON_FONT_COLOR : Color = Color::GRAY;
pub(crate) const LOBBY_DISPLAY_FONT_COLOR   : Color = Color::DARK_GRAY;

//-------------------------------------------------------------------------------------------------------------------

pub(crate) const NUM_LOBBY_CONTENT_ENTRIES : usize = 8;
pub(crate) const LOBBY_LIST_SIZE           : u16   = 10;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct TimeoutConfig
{
    pub ack_request_timeout_ms: u64,
}

//-------------------------------------------------------------------------------------------------------------------
