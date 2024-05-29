use std::collections::HashSet;

use bevy::prelude::*;
use bevy_replicon::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

#[derive(Resource)]
pub struct WatcherMap
{
    /// [ client id  ]
    watchers: HashSet<ClientId>,
}

impl WatcherMap
{
    pub fn new(watchers: HashSet<ClientId>) -> WatcherMap
    {
        WatcherMap { watchers }
    }

    pub fn is_watcher(&self, client_id: ClientId) -> bool
    {
        self.watchers.contains(&client_id)
    }
}

//-------------------------------------------------------------------------------------------------------------------
