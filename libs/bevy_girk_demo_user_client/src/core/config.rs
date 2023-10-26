//local shortcuts

//third-party shortcuts
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

pub(crate) const MENU_BAR_BUTTON      : &'static str = "button.png";
pub(crate) const TEMP_FONT            : &'static str = "fonts/FiraSans-Bold.ttf";
pub(crate) const STATUS_FONT          : &'static str = "fonts/FiraSans-Bold.ttf";
pub(crate) const MENU_BAR_BUTTON_FONT : &'static str = "fonts/FiraSans-Bold.ttf";

pub(crate) const TEMP_FONT_COLOR            : Color = Color::WHITE;
pub(crate) const STATUS_FONT_COLOR          : Color = Color::WHITE;
pub(crate) const MENU_BAR_BUTTON_FONT_COLOR : Color = Color::WHITE;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct TimeoutConfig
{
    pub ack_request_timeout_ms: u64,
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct LobbiesConfig
{
    pub lobby_page_size: u16,
}

//-------------------------------------------------------------------------------------------------------------------
