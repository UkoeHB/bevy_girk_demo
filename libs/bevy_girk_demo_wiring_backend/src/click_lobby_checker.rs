//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_girk_backend_public::*;
use bevy_girk_utils::*;

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ClickLobbyChecker
{
    /// Max number of players allowed in a lobby.
    pub max_lobby_players: u16,
    /// Max number of watchers allowed in a lobby.
    pub max_lobby_watchers: u16,
    /// Min number of players in a lobby required to launch a lobby.
    pub min_players_to_launch: u16,
}

impl ClickLobbyChecker
{
    pub fn count_members(lobby_data: &LobbyData) -> Result<(usize, usize), ()>
    {
        let mut num_players = 0;
        let mut num_watchers = 0;
        for member_data in lobby_data.members.iter().map(|(_, color)| color)
        {
            match ClickLobbyMemberType::try_from(member_data.color)?
            {
                ClickLobbyMemberType::Player  => num_players += 1,
                ClickLobbyMemberType::Watcher => num_watchers += 1,
            }
        }

        Ok((num_players, num_watchers))
    }

    /// Check if the lobby may be hosted by a server.
    pub fn can_launch_hosted(num_players: usize, min_players_to_launch: usize) -> bool
    {
        if num_players < min_players_to_launch { return false; }
        true
    }
}

impl LobbyChecker for ClickLobbyChecker
{
    /// Check if a lobby is semantically valid.
    fn check_lobby(&self, lobby: &Lobby) -> bool
    {
        // excessively large passwords not allowed
        if lobby.get_password().len() > 15 { return false; }

        // custom lobby data must deserialize
        let Some(config) = deser_msg::<ClickLobbyConfig>(&lobby.custom_data()) else { return false; };

        // check that configs are within acceptable bounds
        if config.max_players  > self.max_lobby_players  { return false; }
        if config.max_watchers > self.max_lobby_watchers { return false; }

        // get max count member types
        let Ok((num_players, num_watchers)) = Self::count_members(&lobby.data) else { return false; };

        // check configs
        if num_players  > config.max_players  as usize { return false; }
        if num_watchers > config.max_watchers as usize { return false; }

        true
    }

    /// Check if a new lobby member may be added to a lobby.
    fn allow_new_member(
        &self,
        lobby       : &Lobby,
        member_id   : u128,
        member_data : LobbyMemberData,
        password    : &String,
    ) -> bool
    {
        // check if in lobby already
        if lobby.has_member(member_id) { return false; }

        // check password
        if lobby.get_password() != password { return false; }

        // get member type
        let Ok(member_type) = ClickLobbyMemberType::try_from(member_data.color) else { return false; };

        // count current players and watchers
        let Ok((num_players, num_watchers)) = Self::count_members(&lobby.data) else { return false; };

        // check if the member's type has exceeded lobby capacity
        let Some(config) = deser_msg::<ClickLobbyConfig>(&lobby.custom_data()) else { return false; };

        match member_type
        {
            ClickLobbyMemberType::Player =>
            {
                if num_players >= config.max_players as usize { return false; }
            }
            ClickLobbyMemberType::Watcher =>
            {
                if num_watchers >= config.max_watchers as usize { return false; }
            }
        }

        true
    }

    /// Check if a lobby is launchable.
    fn can_launch(&self, lobby: &Lobby) -> bool
    {
        // count players
        let Ok((num_players, _)) = Self::count_members(&lobby.data) else { return false; };

        Self::can_launch_hosted(num_players, self.min_players_to_launch as usize)
    }
}

//-------------------------------------------------------------------------------------------------------------------
