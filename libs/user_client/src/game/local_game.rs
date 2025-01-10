use bevy::prelude::*;
use bevy_cobweb::prelude::*;

//-------------------------------------------------------------------------------------------------------------------

fn try_take_local_report(
    mut c: Commands,
    mut manager: ResMut<LocalGameManager>,
    config: Option<Res<ClientFwConfig>>,
)
{
    let Some(report) = manager.take_report() else { return };

    match report {
        LocalGameReport::Ended { game_id, report } => {
            tracing::info!("local game {game_id} ended");
            // send out report for use by the app
            c.react().broadcast(report);
        }
        LocalGameReport::Aborted { game_id } => {
            tracing::info!("local game {game_id} aborted");
            // Abort if in the mentioned game.
            if let Some(config) = config {
                if config.game_id() == game_id {
                    c.queue(ClientInstanceCommand::Abort);
                }
            }
        }
    }
}

//-------------------------------------------------------------------------------------------------------------------

pub(super) struct LocalGamePlugin;

impl Plugin for LocalGamePlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(First, try_take_local_report);
    }
}

//-------------------------------------------------------------------------------------------------------------------
