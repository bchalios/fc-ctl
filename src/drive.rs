use clap::{Args, Subcommand};
use fclib::client::drive::{Drive, PartialDrive};
use fclib::client::rate_limiter::RateLimiter;
use fclib::client::ApiClient;

use crate::rate_limiter::{RateLimiterConf, WithRateLimiterConf};
use crate::Result;

#[derive(Debug, Args)]
pub(crate) struct DriveHelper {
    #[clap(flatten)]
    drive: Drive,

    #[clap(flatten)]
    rate_limiter: RateLimiterConf,
}

#[derive(Debug, Args)]
pub(crate) struct PartialDriveHelper {
    #[clap(flatten)]
    drive: PartialDrive,

    #[clap(flatten)]
    rate_limiter: RateLimiterConf,
}

#[derive(Debug, Subcommand)]
pub(crate) enum DriveCmd {
    Add(DriveHelper),
    Update(PartialDriveHelper),
}

impl WithRateLimiterConf for Drive {
    fn set_rate_limiter(&mut self, rate_limiter: RateLimiter) {
        Drive::set_rate_limiter(self, rate_limiter);
    }
}

impl WithRateLimiterConf for PartialDrive {
    fn set_rate_limiter(&mut self, rate_limiter: RateLimiter) {
        PartialDrive::set_rate_limiter(self, rate_limiter);
    }
}

impl DriveCmd {
    pub(crate) async fn parse(self, api_client: &mut ApiClient) -> Result<()> {
        match self {
            DriveCmd::Add(mut drive) => {
                let d = &mut drive.drive;
                drive.rate_limiter.parse_rate_limiter(d);
                api_client.add_drive(d.drive_id(), d).await?;
            }
            DriveCmd::Update(mut drive) => {
                let d = &mut drive.drive;
                drive.rate_limiter.parse_rate_limiter(d);
                api_client.update_drive(d.drive_id(), d).await?;
            }
        }

        Ok(())
    }
}
