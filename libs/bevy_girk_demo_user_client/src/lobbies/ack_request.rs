//local shortcuts
use crate::*;

//third-party shortcuts
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_fn_plugin::bevy_plugin;
use bevy_kot::ecs::*;

//standard shortcuts
use std::time::Duration;

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn focus_window_on_ack_request(
    ack_request : Res<ReactRes<AckRequest>>,
    mut window  : Query<&mut Window, With<PrimaryWindow>>,
){
    // only focus the window when a new ack request is set
    if !ack_request.is_set() { return; }

    // focus the window
    window.single_mut().focused = true;
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn setup_ack_request(mut rcommands: ReactCommands)
{
    rcommands.add_resource_mutation_reactor::<AckRequest>(
            |world| syscall(world, (), focus_window_on_ack_request)
        );
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

fn try_timeout_ack_request(
    mut rcommands   : ReactCommands,
    time            : Res<Time>,
    mut ack_request : ResMut<ReactRes<AckRequest>>,
){
    // check if there is a pending ack request
    if !ack_request.is_set() { return; }

    // update the request timer
    let ack_request_mut = ack_request.get_mut_noreact();
    let timer = &mut ack_request_mut.timer;
    if ack_request_mut.just_reset
    {
        ack_request_mut.just_reset = false;
    }
    else
    {
        timer.tick(time.delta());
    }

    // check if the timer has expired
    if !timer.finished() { return; }

    // clear the ack request
    tracing::trace!("reseting ack request after timeout");
    ack_request.get_mut(&mut rcommands).clear();
}

//-------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------

/// Caches the current pending ack request (if there is one).
///
/// This is a reactive resource.
#[derive(Debug)]
pub(crate) struct AckRequest
{
    /// Lobby id of the current ack request.
    current: Option<u64>,

    /// Timer for the current request. When the timer expires, the request will be reset.
    timer: Timer,
    /// Flag indicating the timer has just been reset. We don't want to tick the timer in the tick where it was reset.
    just_reset: bool,
}

impl AckRequest
{
    pub(crate) fn new(timeout_duration: Duration) -> Self
    {
        Self{ current: None, timer: Timer::new(timeout_duration, TimerMode::Once), just_reset: false }
    }

    pub(crate) fn set(&mut self, new_ack_request: u64)
    {
        self.current = Some(new_ack_request);
        self.timer.reset();
        self.just_reset = true;
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

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn AckRequestPlugin(app: &mut App)
{
    let timer_configs = app.world.resource::<TimerConfigs>();
    let ack_request_timeout = Duration::from_millis(timer_configs.ack_request_timeout_ms);

    app
        .insert_resource(ReactRes::new(AckRequest::new(ack_request_timeout)))
        .add_systems(Startup,
            (
                setup_ack_request,
            )
        )
        .add_systems(PreUpdate,
            (
                try_timeout_ack_request,
            )
        )
        ;
}

//-------------------------------------------------------------------------------------------------------------------
