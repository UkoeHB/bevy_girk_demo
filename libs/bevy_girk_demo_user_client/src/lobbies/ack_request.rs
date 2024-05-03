use std::time::Duration;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_cobweb::prelude::*;
use bevy_fn_plugin::bevy_plugin;

use crate::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Default, Debug)]
enum AckFocusState
{
    #[default]
    None,
    Start,
    Halfway,
}

fn focus_window_for_ack_request(
    mut state: Local<AckFocusState>,
    time: Res<Time>,
    ack_request: ReactRes<AckRequestData>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
)
{
    // try to reset
    if !ack_request.is_set() {
        *state = AckFocusState::None;
        return;
    }

    // focus the window when the request timer starts, and when it is half-way done
    match *state {
        AckFocusState::None => {
            // focus when request was just set
            *state = AckFocusState::Start;
        }
        AckFocusState::Start => {
            // focus when timer moves to halfway point
            let elapsed_since_request = time
                .elapsed()
                .saturating_sub(ack_request.ack_time)
                .as_millis();
            if elapsed_since_request < (ack_request.display_duration().as_millis() / 2) {
                return;
            }

            *state = AckFocusState::Halfway;
        }
        AckFocusState::Halfway => {
            // no more focusing, wait for reset
            return;
        }
    }

    window.single_mut().focused = true;
}

//-------------------------------------------------------------------------------------------------------------------

fn set_ack_request_data(
    mut c: Commands,
    time: Res<Time>,
    events: BroadcastEvent<AckRequest>,
    mut ack_request: ReactResMut<AckRequestData>,
)
{
    let ack_req = events.read().unwrap();
    ack_request
        .get_mut(&mut c)
        .set(ack_req.lobby_id, time.elapsed());
}

//-------------------------------------------------------------------------------------------------------------------

fn setup_ack_request_handlers(mut c: Commands)
{
    c.react()
        .on(broadcast::<AckRequest>(), set_ack_request_data);
    c.react()
        .on(resource_mutation::<AckRequestData>(), focus_window_for_ack_request);
}

//-------------------------------------------------------------------------------------------------------------------

fn try_timeout_ack_request(mut c: Commands, time: Res<Time>, mut ack_request: ReactResMut<AckRequestData>)
{
    // check if there is a pending ack request
    if !ack_request.is_set() {
        return;
    }

    // update the request timer
    // - we don't tick the timer on the first tick after an ack request was received
    // - we trigger reactions here in case listeners care about the timer
    let start_time = ack_request.ack_time;
    let ack_request_mut = ack_request.get_mut(&mut c);
    let timer = &mut ack_request_mut.timer;

    if start_time != time.elapsed() {
        timer.tick(time.delta());
    }

    // check if the timer has expired
    if !timer.finished() {
        return;
    }

    // clear the ack request
    tracing::trace!("resetting ack request after timeout");
    ack_request_mut.clear();
}

//-------------------------------------------------------------------------------------------------------------------

/// Caches the current pending ack request (if there is one).
#[derive(ReactResource, Debug)]
pub(crate) struct AckRequestData
{
    /// Lobby id of the current ack request.
    current: Option<u64>,

    /// Timer for the current request. When the timer expires, the request will be reset.
    timer: Timer,
    /// Amount of time to 'shave off' the timer displayed to users.
    timer_buffer: Duration,
    /// Current time when the ack request was last set. Used by the timer to avoid ticking in the first tick.
    ack_time: Duration,

    /// Indicates if a nack was sent since the request was last set.
    nacked: bool,
    /// Indicates if an ack was sent since the request was last set.
    acked: bool,
}

impl AckRequestData
{
    pub(crate) fn new(timeout_duration: Duration, timer_buffer: Duration) -> Self
    {
        Self {
            current: None,
            timer: Timer::new(timeout_duration, TimerMode::Once),
            timer_buffer,
            ack_time: Duration::default(),
            nacked: false,
            acked: false,
        }
    }

    pub(crate) fn set(&mut self, new_ack_request: u64, ack_time: Duration)
    {
        self.current = Some(new_ack_request);
        self.timer.reset();
        self.ack_time = ack_time;
        self.nacked = false;
        self.acked = false;
    }

    pub(crate) fn set_nacked(&mut self)
    {
        self.nacked = true;
    }

    pub(crate) fn set_acked(&mut self)
    {
        self.acked = true;
    }

    pub(crate) fn clear(&mut self)
    {
        self.current = None;
        self.nacked = false;
        self.acked = false;
    }

    pub(crate) fn get(&self) -> Option<u64>
    {
        self.current
    }

    pub(crate) fn is_set(&self) -> bool
    {
        self.current.is_some()
    }

    pub(crate) fn display_duration(&self) -> Duration
    {
        self.timer.duration().saturating_sub(self.timer_buffer)
    }

    pub(crate) fn time_remaining_for_display(&self) -> Duration
    {
        self.timer.remaining().saturating_sub(self.timer_buffer)
    }

    pub(crate) fn is_nacked(&self) -> bool
    {
        self.nacked
    }

    pub(crate) fn is_acked(&self) -> bool
    {
        self.acked
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// This is a reactive data event used to transmit new ack requests.
#[derive(Debug, Clone)]
pub(crate) struct AckRequest
{
    pub(crate) lobby_id: u64,
}

//-------------------------------------------------------------------------------------------------------------------

#[bevy_plugin]
pub(crate) fn AckRequestPlugin(app: &mut App)
{
    let timer_configs = app.world.resource::<TimerConfigs>();
    let timeout = Duration::from_millis(timer_configs.ack_request_timeout_ms);
    let timer_buffer = Duration::from_millis(timer_configs.ack_request_timer_buffer_ms);

    app.insert_react_resource(AckRequestData::new(timeout, timer_buffer))
        .add_systems(Startup, (setup_ack_request_handlers,))
        .add_systems(PreUpdate, (try_timeout_ack_request,));
}

//-------------------------------------------------------------------------------------------------------------------
