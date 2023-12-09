//local shortcuts
use crate::*;
use bevy_girk_demo_ui_prefab::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot::prelude::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub(crate) struct UserClientBackdrop
{
    pub(crate) plain_box: PlainBox,
}

impl Default for UserClientBackdrop
{
    fn default() -> Self
    {
        Self{ plain_box : PlainBox{ img: BACKDROP } }
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
    client_backdrop  : UserClientBackdrop,
    game_in_progress : GameInProgressStyle,
}

//-------------------------------------------------------------------------------------------------------------------
