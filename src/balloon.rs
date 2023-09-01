use clap::{Args, Subcommand};
use fclib::client::balloon::Balloon;
use fclib::client::ApiClient;

use crate::Result;

#[derive(Debug, Clone, Args)]
pub(crate) struct BalloonArgs {
    #[clap(flatten)]
    balloon: Balloon,
}

#[derive(Debug, Clone, Args)]
pub(crate) struct BalloonUpdateArgs {
    size_mib: i32,
}

#[derive(Debug, Clone, Args)]
pub(crate) struct BalloonStatsUpdateArgs {
    stats_interval_sec: i32,
}

/// Manage Balloon device
#[derive(Clone, Debug, Subcommand)]
pub(crate) enum BalloonCmd {
    /// Configure the balloon device
    Config(BalloonArgs),
    /// Update configuration of the balloon device
    Update(BalloonUpdateArgs),
    /// Update configuration of balloon stats
    StatsUpdate(BalloonStatsUpdateArgs),
}

impl BalloonCmd {
    pub(crate) async fn parse(self, api_client: &mut ApiClient) -> Result<()> {
        match self {
            BalloonCmd::Config(balloon) => {
                api_client.configure_balloon(&balloon.balloon).await?;
            }
            BalloonCmd::Update(balloon) => {
                api_client.update_balloon_size(balloon.size_mib).await?;
            }
            BalloonCmd::StatsUpdate(balloon) => {
                api_client
                    .update_balloon_stats_interval(balloon.stats_interval_sec)
                    .await?;
            }
        }

        Ok(())
    }
}
