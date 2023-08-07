use clap::Args;
use fclib::client::kernel::BootSource;
use fclib::client::ApiClient;

use crate::Result;

/// Configure microVM guest kernel
#[derive(Debug, Args)]
pub(crate) struct BootSourceArgs {
    #[clap(flatten)]
    kernel: BootSource,
}

pub(crate) async fn parse(api_client: &mut ApiClient, args: &BootSourceArgs) -> Result<()> {
    api_client.set_boot_source(&args.kernel).await?;
    Ok(())
}
