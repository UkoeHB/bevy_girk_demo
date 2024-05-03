use std::fmt::Write;

use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_girk_demo_ui_prefab::*;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

/// Set a button's text to `...` when it is waiting for a pending request.
pub(crate) fn setup_simple_pending_button_text<Tag: Component>(
    In((button_entity, default_text)): In<(Entity, &'static str)>,
    mut c: Commands,
    marker: Query<Entity, With<Tag>>,
)
{
    let marker_entity = marker.single();

    // when a request starts
    c.react().on(
        entity_insertion::<PendingRequest>(marker_entity),
        move |mut text: TextHandle| {
            text.write(button_entity, 0, |text| write!(text, "{}", "..."))
                .unwrap()
        },
    );

    // when a request completes
    c.react().on(
        entity_removal::<PendingRequest>(marker_entity),
        move |mut text: TextHandle| {
            text.write(button_entity, 0, |text| write!(text, "{}", default_text))
                .unwrap()
        },
    );
}

//-------------------------------------------------------------------------------------------------------------------
