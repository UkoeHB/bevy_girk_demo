use bevy::prelude::*;
use bevy_kot::prelude::*;
use ui_prefab::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct GameClientBackdrop
{
    pub(crate) plain_box: PlainBox,
}

impl Default for GameClientBackdrop
{
    fn default() -> Self
    {
        Self { plain_box: PlainBox { img: BACKDROP } }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct GameInitializingStyle
{
    pub(crate) background_img: (&'static str, Vec2),
    pub(crate) background_color: Color,
    pub(crate) text: BasicText,

    pub(crate) loading_bar_box_img: (&'static str, Vec2),
    pub(crate) loading_bar_box_color: Color,

    pub(crate) loading_bar_img: (&'static str, Vec2),
    pub(crate) loading_bar_color: Color,
}

impl Default for GameInitializingStyle
{
    fn default() -> Self
    {
        Self {
            background_img: BOX,
            background_color: Color::DARK_GRAY,
            text: basic_text_default_light_style(),

            loading_bar_box_img: BOX,
            loading_bar_box_color: Color::WHITE,

            loading_bar_img: FILM,
            loading_bar_color: Color::BLACK,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct GameScoreboardStyle
{
    pub(crate) text: BasicText,
}

impl Default for GameScoreboardStyle
{
    fn default() -> Self
    {
        Self { text: basic_text_default_light_style() }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct GameClickerStyle
{
    pub(crate) button: BasicButton,
}

impl Default for GameClickerStyle
{
    fn default() -> Self
    {
        Self {
            button: BasicButton {
                default_img: MISC_BUTTON,
                pressed_img: MISC_BUTTON,
                default_img_color: Color::hsl(360., 1., 0.30),
                pressed_img_color: Color::hsl(360., 1., 0.20),
                text: basic_button_default_text_style(),
            },
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct GameOverStyle
{
    pub(crate) background_img: (&'static str, Vec2),
    pub(crate) background_color: Color,
    pub(crate) text: BasicText,
}

impl Default for GameOverStyle
{
    fn default() -> Self
    {
        Self {
            background_img: FILM,
            background_color: Color::WHITE,
            text: basic_text_default_light_style(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(StyleBundle, Default, Debug)]
pub(crate) struct ClientSkinStyles
{
    prefabs: BasicPrefabStyles,
    backdrop: GameClientBackdrop,
    game_in_progress: GameInitializingStyle,
    game_scoreboard: GameScoreboardStyle,
    game_clicker: GameClickerStyle,
    game_over: GameOverStyle,
}

//-------------------------------------------------------------------------------------------------------------------
