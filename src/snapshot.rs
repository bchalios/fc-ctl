use clap::{Args, Subcommand};
use fclib::client::snapshot::{SnapshotCreateParams, SnapshotLoadParams};
use fclib::client::ApiClient;

use crate::Result;

/// Take a snapshot of the microVM
///
/// This will first pause the microVM and then create the snapshot.
/// It allows to, optionally, resume the microVM after taking the snapshot.
#[derive(Debug, Args)]
pub(crate) struct SnapshotCreateWrapper {
    #[clap(flatten)]
    snap_args: SnapshotCreateParams,

    /// resume the microVM upon loading snapshot
    #[arg(short, long)]
    resume_vm: bool,
}

/// microVM snapshot operations
#[derive(Debug, Subcommand)]
pub(crate) enum SnapshotCmd {
    /// Create a microVM snapshot
    Create(SnapshotCreateWrapper),
    /// Load a microVM snapshot
    Load(SnapshotLoadParams),
}

impl SnapshotCmd {
    pub(crate) async fn parse(&self, api_client: &ApiClient) -> Result<()> {
        match self {
            SnapshotCmd::Create(params) => {
                api_client.pause_microvm().await?;
                api_client.snapshot_microvm(&params.snap_args).await?;
                if params.resume_vm {
                    api_client.resume_microvm().await?;
                }
            }
            SnapshotCmd::Load(params) => {
                api_client.load_microvm_snapshot(params).await?;
                if params.enabled_resume_vm() {
                    api_client.resume_microvm().await?;
                }
            }
        }

        Ok(())
    }
}
