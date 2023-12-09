//local shortcuts
use crate::*;
use bevy_girk_demo_ui_prefab::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct UserClientBackdropStyle
{
    pub(crate) backdrop_box: PlainBox,
}

impl Default for UserClientBackdropStyle
{
    fn default() -> Self
    {
        Self{ backdrop_box : PlainBox{ img: CLIENT_BACKDROP } }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct PlayButtonStyle
{
    pub default_img: (&'static str, Vec2),
    pub pressed_img: (&'static str, Vec2),

    pub default_img_color_play: Color,
    pub pressed_img_color_play: Color,

    pub default_img_color_inlobby: Color,
    pub pressed_img_color_inlobby: Color,

    pub text: BasicText,
}

impl Default for PlayButtonStyle
{
    fn default() -> Self
    {
        Self{ 
            default_img: MENU_BAR_BUTTON,
            pressed_img: MENU_BAR_BUTTON,
            default_img_color_play: Color::hsl(94., 1., 0.35),
            pressed_img_color_play: Color::hsl(94., 1., 0.22),
            default_img_color_inlobby: Color::hsl(180., 1., 0.35),
            pressed_img_color_inlobby: Color::hsl(180., 1., 0.22),
            text: basic_text_default_light_style(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct MenuButtonStyle
{
    pub default_img: (&'static str, Vec2),
    pub pressed_img: (&'static str, Vec2),

    pub default_img_color: Color,
    pub pressed_img_color: Color,

    pub text: BasicText,
}

impl Default for MenuButtonStyle
{
    fn default() -> Self
    {
        Self{ 
            default_img: MENU_BAR_BUTTON,
            pressed_img: MENU_BAR_BUTTON,
            default_img_color: Color::hsl(32., 1., 0.35),
            pressed_img_color: Color::hsl(32., 1., 0.22),
            text: basic_text_default_light_style(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct LobbyDisplayStyle
{
    pub(crate) backdrop_box: PlainBox,
}

impl Default for LobbyDisplayStyle
{
    fn default() -> Self
    {
        Self{ backdrop_box : PlainBox{ img: LOBBY_BACKDROP } }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct LobbyListStyle
{
    pub(crate) backdrop_box: PlainBox,
}

impl Default for LobbyListStyle
{
    fn default() -> Self
    {
        Self{ backdrop_box : PlainBox{ img: LOBBY_BACKDROP } }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct GameInProgressStyle
{
    pub(crate) background_img: (&'static str, Vec2),
    pub(crate) background_color: Color,
    pub(crate) text: BasicText,
}

impl Default for GameInProgressStyle
{
    fn default() -> Self
    {
        Self{
            background_img   : BOX,
            background_color : Color::BLACK,
            text             : basic_text_default_light_style(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(StyleBundle, Default, Debug)]
pub(crate) struct UserClientStyles
{
    prefabs          : BasicPrefabStyles,
    client_backdrop  : UserClientBackdropStyle,
    play_button      : PlayButtonStyle,
    menu_buttons     : MenuButtonStyle,
    lobby_display    : LobbyDisplayStyle,
    lobby_list       : LobbyListStyle,
    game_in_progress : GameInProgressStyle,
}

impl UserClientStyles
{
    pub(crate) fn new() -> Self
    {
        // customize defaults
        let mut default: Self = Default::default();
        default.prefabs.basic_popup.backdrop = POPUP_BACKDROP;
        default.prefabs.basic_popup.backdrop_color = Color::WHITE;

        default
    }
}

//-------------------------------------------------------------------------------------------------------------------
