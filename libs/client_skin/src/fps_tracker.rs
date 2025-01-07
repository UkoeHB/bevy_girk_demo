use std::collections::VecDeque;
use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimeSystem;
use bevy_cobweb::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

//todo: pass by value on construction
const FPS_TRACKER_NUM_RECORDS: u8 = 30;

//-------------------------------------------------------------------------------------------------------------------

/// Update FPS tracker with new time.
fn update_fps_tracker(mut rc: ReactCommands, mut tracker: ReactResMut<FpsTracker>, time: Res<Time>)
{
    tracker
        .get_mut(&mut rc)
        .update(time.delta_seconds(), time.elapsed());
}

//-------------------------------------------------------------------------------------------------------------------

/// Reactive resource for tracking FPS.
#[derive(ReactResource)]
pub struct FpsTracker
{
    max_records: u8,
    delta_record_ns: VecDeque<u64>,
    total_delta_ns: u64,
    previous_time: Duration,
    current_time: Duration,
}

impl FpsTracker
{
    /// Make new tracker.
    pub fn new(max: u8) -> Self
    {
        Self {
            max_records: max.max(1),
            delta_record_ns: VecDeque::with_capacity((max + 1) as usize),
            total_delta_ns: 0u64,
            previous_time: Duration::from_secs(0),
            current_time: Duration::from_secs(0),
        }
    }

    /// Average delta recorded (in seconds).
    pub fn average_delta_seconds(&self) -> f32
    {
        (self.average_delta_nanoseconds() as f32) / 1_000_000_000.0
    }

    /// Average delta recorded (in nanoseconds).
    pub fn average_delta_nanoseconds(&self) -> u64
    {
        match self.delta_record_ns.len() {
            0 => 0u64,
            num_records => self.total_delta_ns / (num_records as u64),
        }
    }

    /// Get FPS estimate.
    pub fn fps(&self) -> u16
    {
        match 1_000_000_000u64.checked_div(self.average_delta_nanoseconds()) {
            Some(rate) => rate as u16,
            None => 0,
        }
    }

    pub fn previous_time(&self) -> Duration
    {
        self.previous_time
    }
    pub fn current_time(&self) -> Duration
    {
        self.current_time
    }

    /// Update the tracker with a new time.
    pub fn update(self: &mut FpsTracker, delta: f32, current_time: Duration)
    {
        // 1. add new record
        let delta_ns = (delta * 1_000_000_000.0) as u64;
        self.delta_record_ns.push_back(delta_ns);
        self.total_delta_ns += delta_ns;

        // 2. update current time
        self.previous_time = self.current_time;
        self.current_time = current_time;

        // 3. remove excess records
        while self.delta_record_ns.len() > (self.max_records as usize) {
            self.total_delta_ns -= self.delta_record_ns.get(0).unwrap();
            self.delta_record_ns.pop_front();
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

/// System set in `First` where the [`FpsTracker`] resource is updated.
#[derive(SystemSet, PartialEq, Eq, Debug, Hash, Clone)]
pub struct FpsTrackerSet;

//-------------------------------------------------------------------------------------------------------------------

/// Tracks FPS. Use the [`FpsTracker`] resource to access the fps with [`FpsTracker::fps()`].
pub(super) struct FpsTrackerPlugin;

impl Plugin for FpsTrackerPlugin
{
    pub fn build(&self, app: &mut App)
    {
        app.insert_react_resource(FpsTracker::new(FPS_TRACKER_NUM_RECORDS))
            .configure_sets(First, FpsTrackerSet.after(TimeSystem))
            .add_systems(First, update_fps_tracker.in_set(FpsTrackerSet));
    }
}

//-------------------------------------------------------------------------------------------------------------------
