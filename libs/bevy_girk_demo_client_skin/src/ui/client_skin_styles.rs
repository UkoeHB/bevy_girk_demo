//local shortcuts
use crate::*;
use bevy_girk_demo_ui_prefab::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::prelude::*;

//standard shortcuts


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
        Self{
            background_img   : BOX,
            background_color : Color::DARK_GRAY,
            text             : basic_text_default_light_style(),

            loading_bar_box_img   : BOX,
            loading_bar_box_color : Color::WHITE,

            loading_bar_img   : FILM,
            loading_bar_color : Color::BLACK,
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
        Self{
            background_img   : FILM,
            background_color : Color::WHITE,
            text             : basic_text_default_light_style(),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(StyleBundle, Default, Debug)]
pub(crate) struct ClientSkinStyles
{
    prefabs          : BasicPrefabStyles,
    game_in_progress : GameInitializingStyle,
    game_over        : GameOverStyle,
}

//-------------------------------------------------------------------------------------------------------------------
