use clap::Subcommand;
use fclib::client::snapshot::{SnapshotCreateParams, SnapshotLoadParams};
use fclib::client::ApiClient;

use crate::Result;

#[derive(Debug, Subcommand)]
pub(crate) enum SnapshotCmd {
    /// Create a microVM snapshot
    Create(SnapshotCreateParams),
    /// Load a microVM snapshot
    Load(SnapshotLoadParams),
}

impl SnapshotCmd {
    pub(crate) async fn parse(&self, api_client: &ApiClient) -> Result<()> {
        match self {
            SnapshotCmd::Create(params) => api_client.snapshot_microvm(params).await?,
            SnapshotCmd::Load(params) => api_client.load_microvm_snapshot(params).await?,
        }

        Ok(())
    }
}
