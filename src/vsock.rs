use clap::Args;
use fclib::client::vsock::Vsock;
use fclib::client::ApiClient;

use crate::Result;

/// Configure vsock device
#[derive(Debug, Args)]
pub(crate) struct VsockArgs {
    #[clap(flatten)]
    vsock: Vsock,
}

pub(crate) async fn parse(api_client: &mut ApiClient, args: &VsockArgs) -> Result<()> {
    api_client.config_vsock(&args.vsock).await?;
    Ok(())
}
