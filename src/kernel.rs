use clap::Subcommand;
use fclib::client::kernel::BootSource;
use fclib::client::ApiClient;

use crate::Result;

#[derive(Debug, Subcommand)]
pub(crate) enum BootSourceCmd {
    Set(BootSource),
}

impl BootSourceCmd {
    pub(crate) async fn parse(&self, api_client: &mut ApiClient) -> Result<()> {
        match self {
            Self::Set(source) => api_client.set_boot_source(source).await?,
        }

        Ok(())
    }
}
