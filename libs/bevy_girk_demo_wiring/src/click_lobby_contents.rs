//local shortcuts
use crate::*;

//third-party shortcuts
use bevy_girk_backend_public::*;
use bevy_girk_utils::*;
use serde::{Deserialize, Serialize};

//standard shortcuts


//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickLobbyConfig
{
    /// Max players allowed in the lobby.
    pub max_players: u16,
    /// Max watchers allowed in the lobby.
    pub max_watchers: u16,
}

impl ClickLobbyConfig
{
    pub fn is_single_player(&self) -> bool
    {
        self.max_players == 1 &&
        self.max_watchers == 0
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ClickLobbyMemberType
{
    Player,
    Watcher,
}

impl TryFrom<LobbyMemberColor> for ClickLobbyMemberType
{
    type Error = ();

    fn try_from(color: LobbyMemberColor) -> Result<ClickLobbyMemberType, ()>
    {
        match color.0
        {
            0u64 => Ok(ClickLobbyMemberType::Player),
            1u64 => Ok(ClickLobbyMemberType::Watcher),
            _    => Err(())
        }
    }
}

impl Into<LobbyMemberColor> for ClickLobbyMemberType
{
    fn into(self) -> LobbyMemberColor
    {
        match self
        {
            ClickLobbyMemberType::Player  => LobbyMemberColor(0u64),
            ClickLobbyMemberType::Watcher => LobbyMemberColor(1u64),
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ClickLobbyContents
{
    /// This lobby's id.
    pub id: u64,
    /// The id of this lobby's owner.
    pub owner_id: u128,

    /// Lobby config.
    pub config: ClickLobbyConfig,

    /// Players in this lobby.
    pub players: Vec<(bevy_simplenet::EnvType, u128)>,
    /// Watchers in this lobby.
    pub watchers: Vec<(bevy_simplenet::EnvType, u128)>,
}

impl ClickLobbyContents
{
    pub fn get_id(&self, member_type: ClickLobbyMemberType, idx: usize) -> Option<&u128>
    {
        match member_type
        {
            ClickLobbyMemberType::Player  => self.players.get(idx).map(|(_, id)| id),
            ClickLobbyMemberType::Watcher => self.watchers.get(idx).map(|(_, id)| id),
        }
    }

    pub fn num(&self, member_type: ClickLobbyMemberType) -> usize
    {
        match member_type
        {
            ClickLobbyMemberType::Player  => self.players.len(),
            ClickLobbyMemberType::Watcher => self.watchers.len(),
        }
    }

    pub fn max(&self, member_type: ClickLobbyMemberType) -> u16
    {
        match member_type
        {
            ClickLobbyMemberType::Player  => self.config.max_players,
            ClickLobbyMemberType::Watcher => self.config.max_watchers,
        }
    }

    /// Check if the game can be launched while hosted.
    pub fn can_launch_hosted(&self) -> bool
    {
        ClickLobbyChecker::can_launch_hosted(self.players.len(), MIN_PLAYERS_TO_LAUNCH)
    }
}

impl TryFrom<LobbyData> for ClickLobbyContents
{
    type Error = ();

    fn try_from(data: LobbyData) -> Result<Self, Self::Error>
    {
        // config
        let config = deser_msg::<ClickLobbyConfig>(&data.serialized_custom_data).ok_or(())?;

        // members
        let mut players  = Vec::default();
        let mut watchers = Vec::default();
        for (user_id, member_data) in data.members.iter()
        {
            match ClickLobbyMemberType::try_from(member_data.color)?
            {
                ClickLobbyMemberType::Player  => players.push((member_data.env, *user_id)),
                ClickLobbyMemberType::Watcher => watchers.push((member_data.env, *user_id)),
            }
        }

        Ok(Self{
            id       : data.id,
            owner_id : data.owner_id,
            config,
            players,
            watchers
        })   
    }
}

//-------------------------------------------------------------------------------------------------------------------
