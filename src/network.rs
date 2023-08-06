use clap::{Args, Subcommand};
use fclib::client::network::{NetworkInterface, PartialNetworkInterface};
use fclib::client::rate_limiter::RateLimiter;
use fclib::client::ApiClient;

use crate::rate_limiter::RateLimiterConf;
use crate::Result;

#[derive(Debug, Args)]
pub(crate) struct TxRateLimiterConf {
    /// Rate limiter configuration for operations per second on the TX queue.
    /// Values [token_bucket_size, refill_time_in_msec, (initial_burst_size)]
    #[arg(long, num_args(2..4))]
    tx_ops: Option<Vec<i64>>,

    /// Rate limiter configuration for bandwidth on the TX queue.
    /// Values [token_bucket_size, refill_time_in_msec, (initial_burst_size)]
    #[arg(long, num_args(2..4))]
    tx_bw: Option<Vec<i64>>,
}

#[derive(Debug, Args)]
pub(crate) struct RxRateLimiterConf {
    /// Rate limiter configuration for operations per second on the RX queue.
    /// Values [token_bucket_size, refill_time_in_msec, (initial_burst_size)]
    #[arg(long, num_args(2..4))]
    rx_ops: Option<Vec<i64>>,

    /// Rate limiter configuration for bandwidth on the RX queue.
    /// Values [token_bucket_size, refill_time_in_msec, (initial_burst_size)]
    #[arg(long, num_args(2..4))]
    rx_bw: Option<Vec<i64>>,
}

#[derive(Debug, Args)]
struct NetRateLimiterConf {
    #[clap(flatten)]
    tx_rate_limiter: TxRateLimiterConf,
    #[clap(flatten)]
    rx_rate_limiter: RxRateLimiterConf,
}

trait WithNetRateLimiterConf {
    fn set_tx_rate_limiter(&mut self, rate_limiter: RateLimiter);
    fn set_rx_rate_limiter(&mut self, rate_limiter: RateLimiter);
}

impl WithNetRateLimiterConf for NetworkInterface {
    fn set_tx_rate_limiter(&mut self, rate_limiter: RateLimiter) {
        self.set_rx_rate_limiter(rate_limiter);
    }

    fn set_rx_rate_limiter(&mut self, rate_limiter: RateLimiter) {
        self.set_tx_rate_limiter(rate_limiter);
    }
}

impl WithNetRateLimiterConf for PartialNetworkInterface {
    fn set_tx_rate_limiter(&mut self, rate_limiter: RateLimiter) {
        self.set_rx_rate_limiter(rate_limiter);
    }

    fn set_rx_rate_limiter(&mut self, rate_limiter: RateLimiter) {
        self.set_tx_rate_limiter(rate_limiter);
    }
}

impl NetRateLimiterConf {
    fn parse_rate_limiters(&self, net: &mut dyn WithNetRateLimiterConf) -> Result<()> {
        let mut rate_limiter = RateLimiter::new();
        if let Some(ops) = &self.tx_rate_limiter.tx_ops {
            rate_limiter.set_ops(RateLimiterConf::parse_token_bucket(ops));
        }
        if let Some(bw) = &self.tx_rate_limiter.tx_bw {
            rate_limiter.set_bandwidth(RateLimiterConf::parse_token_bucket(bw));
        }
        net.set_tx_rate_limiter(rate_limiter.clone());

        if let Some(ops) = &self.rx_rate_limiter.rx_ops {
            rate_limiter.set_ops(RateLimiterConf::parse_token_bucket(ops));
        }
        if let Some(bw) = &self.rx_rate_limiter.rx_bw {
            rate_limiter.set_bandwidth(RateLimiterConf::parse_token_bucket(bw));
        }
        net.set_rx_rate_limiter(rate_limiter);

        Ok(())
    }
}

#[derive(Debug, Args)]
pub(crate) struct NetworkInterfaceHelper {
    #[clap(flatten)]
    net: NetworkInterface,
    #[clap(flatten)]
    rate_limiter: NetRateLimiterConf,
}

#[derive(Debug, Args)]
pub(crate) struct PartialNetworkInterfaceHelper {
    #[clap(flatten)]
    net: PartialNetworkInterface,
    #[clap(flatten)]
    rate_limiter: NetRateLimiterConf,
}

#[derive(Debug, Subcommand)]
pub(crate) enum NetCommand {
    /// Attach a new network interface to the microVM
    Add(NetworkInterfaceHelper),
    /// Update a network interface
    Update(PartialNetworkInterfaceHelper),
}

impl NetCommand {
    pub(crate) async fn parse(self, api_client: &mut ApiClient) -> Result<()> {
        match self {
            Self::Add(mut net) => {
                let n = &mut net.net;
                net.rate_limiter.parse_rate_limiters(n)?;
                api_client.add_network_interface(n.iface_id(), n).await?;
            }
            Self::Update(mut net) => {
                let n = &mut net.net;
                net.rate_limiter.parse_rate_limiters(n)?;
                api_client.update_network_interface(n.iface_id(), n).await?;
            }
        }

        Ok(())
    }
}
