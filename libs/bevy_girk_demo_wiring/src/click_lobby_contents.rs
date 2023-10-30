//local shortcuts
use bevy_girk_backend_public::*;

//third-party shortcuts

//standard shortcuts


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
    /// Players in this lobby.
    pub players: Vec<(bevy_simplenet::EnvType, u128)>,
    /// Watchers in this lobby.
    pub watchers: Vec<(bevy_simplenet::EnvType, u128)>,
}

impl TryFrom<LobbyData> for ClickLobbyContents
{
    type Error = ();

    fn try_from(data: LobbyData) -> Result<Self, Self::Error>
    {
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

        Ok(Self{ id: data.id, owner_id: data.owner_id, players, watchers })   
    }
}

//-------------------------------------------------------------------------------------------------------------------
