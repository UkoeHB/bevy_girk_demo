//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy_kot_derive::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub struct BasicText
{
    pub color: Color,
    pub font: &'static str,
}

impl Default for BasicText
{
    fn default() -> Self
    {
        Self{
            color: Color::BLACK,
            font: BASIC_FONT,
        }
    }
}

pub fn basic_text_default_light_style() -> BasicText
{
    BasicText{
            color: Color::WHITE,
            font: BASIC_FONT,
        }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub struct BasicButton
{
    pub default_img: (&'static str, Vec2),
    pub pressed_img: (&'static str, Vec2),

    pub default_img_color: Color,
    pub pressed_img_color: Color,

    pub text: BasicText,
}

impl Default for BasicButton
{
    fn default() -> Self
    {
        Self{
            default_img: MISC_BUTTON,
            pressed_img: MISC_BUTTON,
            default_img_color: Color::GRAY,
            pressed_img_color: Color::DARK_GRAY,
            text: basic_button_default_text_style(),
        }
    }
}

pub fn basic_button_default_text_style() -> BasicText
{
    basic_text_default_light_style()
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub struct BasicButtonAvailability
{
    pub active: Color,
    pub inactive: Color,
}

impl Default for BasicButtonAvailability
{
    fn default() -> Self
    {
        Self{
            active: basic_button_default_text_style().color,
            inactive: Color::GRAY,
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Style, Clone, Debug)]
pub struct BasicPopup
{
    /// Style for the cancel/accept buttons.
    pub button: BasicButton,
    /// Image stretched across the screen behind the popup.
    pub background: (&'static str, Vec2),
    /// Image for the window.
    pub window: (&'static str, Vec2),
    /// Color adjustment for the window (applied to window image).
    pub window_color: Color,

    /// Proportional share of the OS window assigned to the popup.
    pub proportions: Vec2,
    /// Vertical share of the window provided to the caller.
    pub content_percent: f32,
    /// Spacing for buttons: |<-spacing->[button]<-gap/2-><-spacing-><-gap/2->[button]<-spacing->|
    pub button_spacing: f32,
    /// Gap between buttons: |<-spacing->[button]<-gap/2-><-spacing-><-gap/2->[button]<-spacing->|
    pub button_gap: f32,
    /// Ratio y/x between button dimensions.
    pub button_ratio: f32,
    /// Extra vertical spacing between bottom of window and area where buttons can be placed.
    ///
    /// Calculated as percent of non-content section at the base of the window.
    pub button_dead_space: f32,
}

impl Default for BasicPopup
{
    fn default() -> Self
    {
        Self{
            button: BasicButton::default(),
            background: FILM,
            window: BOX,
            window_color: Color::AZURE,
            proportions: Vec2{ x: 80.0, y: 80.0 },
            content_percent: 82.0,
            button_spacing: 15.0,
            button_gap: 31.0,
            button_ratio: 0.675,
            button_dead_space: 11.0,
        }
    }
}

pub fn basic_popup_default_button_style() -> BasicButton
{
    BasicButton::default()
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(StyleBundle, Default, Debug)]
pub struct BasicPrefabStyles
{
    basic_text: BasicText,
    basic_button: BasicButton,
    basic_button_availability: BasicButtonAvailability,
    basic_popup: BasicPopup,
}

//-------------------------------------------------------------------------------------------------------------------
