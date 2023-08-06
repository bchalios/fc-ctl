use clap::Args;
use fclib::client::entropy::EntropyDevice;
use fclib::client::rate_limiter::RateLimiter;
use fclib::client::ApiClient;

use crate::rate_limiter::{RateLimiterConf, WithRateLimiterConf};
use crate::Result;

#[derive(Debug, Args)]
pub(crate) struct EntropyArgs {
    #[clap(flatten)]
    rate_limiter: RateLimiterConf,
}

impl WithRateLimiterConf for EntropyDevice {
    fn set_rate_limiter(&mut self, rate_limiter: RateLimiter) {
        EntropyDevice::set_rate_limiter(self, rate_limiter);
    }
}

pub(crate) async fn parse(api_client: &mut ApiClient, args: &EntropyArgs) -> Result<()> {
    let mut entropy = EntropyDevice::new();
    args.rate_limiter.parse_rate_limiter(&mut entropy);
    api_client.configure_entropy_device(&entropy).await?;
    Ok(())
}
