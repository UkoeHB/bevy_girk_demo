//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::window::PrimaryWindow;
use bevy_fn_plugin::bevy_plugin;
use bevy_kot::ecs::*;

//standard shortcuts
use std::time::Duration;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_ack_request(mut rcommands: ReactCommands)
{
    rcommands.add_resource_mutation_reactor::<AckRequest>(
            |world| { syscall(world, (), focus_window_on_ack_request); }
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn focus_window_on_ack_request(
    ack_request : Res<ReactRes<AckRequest>>,
    mut window  : Query<&mut Window, With<PrimaryWindow>>,
){
    // only focus window when a new ack request is set
    if !ack_request.is_set() { return; }

    // focus the window
    window.single_mut().focused = true;
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn ack_request_cleanup(
    mut rcommands   : ReactCommands,
    mut ack_request : ResMut<ReactRes<AckRequest>>,
){
    // clear ack request
    ack_request.get_mut(&mut rcommands).clear();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub(crate) struct AckRequest
{
    current: Option<u64>,
}

impl AckRequest
{
    pub(crate) fn set(&mut self, new_ack_request: u64)
    {
        self.current = Some(new_ack_request);
    }

    pub(crate) fn clear(&mut self)
    {
        self.current = None;
    }

    pub(crate) fn get(&self) -> Option<u64>
    {
        self.current
    }

    pub(crate) fn is_set(&self) -> bool
    {
        self.current.is_some()
    }
}

impl Default for AckRequest { fn default() -> Self { Self{ current: None } } }

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn AckRequestPlugin(app: &mut App)
{
    let timer_configs = app.world.resource::<TimeoutConfig>();
    let ack_request_timeout = Duration::from_millis(timer_configs.ack_request_timeout_ms);

    app
        .insert_resource(ReactRes::new(AckRequest::default()))
        .add_systems(Startup,
            (
                setup_ack_request,
            )
        )
        .add_systems(PostUpdate,
            (
                ack_request_cleanup, apply_deferred,
            ).chain().run_if(on_timer(ack_request_timeout))
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
