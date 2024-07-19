mod balloon;
mod drive;
mod entropy;
mod kernel;
mod machine_config;
mod network;
mod rate_limiter;
mod snapshot;
mod vm_state;
mod vsock;

use balloon::BalloonCmd;
use clap::{Parser, Subcommand};
use drive::DriveCmd;
use entropy::EntropyArgs;
use fclib::client::{ApiClient, FcClientError};
use kernel::BootSourceArgs;
use machine_config::MachineConfigCmd;
use network::NetCommand;
use snapshot::SnapshotCmd;
use vm_state::VmStateCmd;
use vsock::VsockArgs;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Argument error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API Client error: {0}")]
    ApiClient(#[from] FcClientError),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Parser)]
#[command(name = "fc-ctl")]
struct Cli {
    /// Path to Firecracker API socket.
    #[arg(short, long, default_value = "/tmp/firecracker.socket")]
    api_sock: String,

    /// Command to execute.
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(subcommand)]
    Drive(DriveCmd),
    #[command(subcommand)]
    Net(NetCommand),
    #[command(subcommand)]
    MachineConfig(MachineConfigCmd),
    Kernel(BootSourceArgs),
    #[command(subcommand)]
    Microvm(VmStateCmd),
    #[command(subcommand)]
    Snapshot(SnapshotCmd),
    Entropy(EntropyArgs),
    #[command(subcommand)]
    Balloon(BalloonCmd),
    Vsock(VsockArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let mut api_client = ApiClient::new(args.api_sock);
    match args.command {
        Commands::Drive(cmd) => cmd.parse(&mut api_client).await?,
        Commands::MachineConfig(cmd) => cmd.parse(&mut api_client).await?,
        Commands::Net(cmd) => cmd.parse(&mut api_client).await?,
        Commands::Kernel(args) => kernel::parse(&mut api_client, &args).await?,
        Commands::Microvm(cmd) => cmd.parse(&api_client).await?,
        Commands::Snapshot(cmd) => cmd.parse(&api_client).await?,
        Commands::Entropy(args) => entropy::parse(&mut api_client, &args).await?,
        Commands::Balloon(cmd) => cmd.parse(&mut api_client).await?,
        Commands::Vsock(args) => vsock::parse(&mut api_client, &args).await?,
    }

    Ok(())
}
