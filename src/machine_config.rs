use clap::Subcommand;
use fclib::client::vm::MachineConfiguration;
use fclib::client::ApiClient;

use crate::Result;

/// Configure architectural characteristics of the microVM
#[derive(Debug, Subcommand)]
pub(crate) enum MachineConfigCmd {
    Config(MachineConfiguration),
    Get,
    Update(MachineConfiguration),
}

impl MachineConfigCmd {
    pub(crate) async fn parse(&self, api_client: &mut ApiClient) -> Result<()> {
        match self {
            MachineConfigCmd::Config(config) => api_client.configure_machine(config).await?,
            MachineConfigCmd::Get => {
                let config = api_client.get_machine_configuration().await?;
                let json = serde_json::to_string_pretty(&config)?;
                println!("{json}");
            }
            MachineConfigCmd::Update(config) => {
                api_client.update_machine_configuration(config).await?
            }
        }

        Ok(())
    }
}
