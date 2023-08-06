use clap::Subcommand;
use fclib::client::ApiClient;

use crate::Result;

/// Control the state of a microVM
#[derive(Debug, Subcommand)]
pub(crate) enum VmStateCmd {
    /// Start the microVM
    Start,
    /// Shutdown the microVM
    Shutdown,
    /// Pause microVMs vCPUs
    Pause,
    /// Resume microVMs vCPUs
    Resume,
}

impl VmStateCmd {
    pub(crate) async fn parse(&self, api_client: &ApiClient) -> Result<()> {
        match self {
            VmStateCmd::Start => api_client.start_microvm().await?,
            VmStateCmd::Shutdown => api_client.stop_microvm().await?,
            VmStateCmd::Pause => api_client.pause_microvm().await?,
            VmStateCmd::Resume => api_client.resume_microvm().await?,
        }

        Ok(())
    }
}
