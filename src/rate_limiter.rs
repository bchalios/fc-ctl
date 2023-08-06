use clap::Args;
use fclib::client::rate_limiter::{RateLimiter, TokenBucket};

pub trait WithRateLimiterConf {
    fn set_rate_limiter(&mut self, rate_limiter: RateLimiter);
}

#[derive(Debug, Args)]
pub(crate) struct RateLimiterConf {
    /// Rate limiter configuration for operations per second.
    /// Values [token_bucket_size, refill_time_in_msec, (initial_burst_size)]
    #[arg(long, num_args(2..4))]
    ops: Option<Vec<i64>>,

    /// Rate limiter configuration for bandwidth.
    /// Values [token_bucket_size, refill_time_in_msec, (initial_burst_size)]
    #[arg(long, num_args(2..4))]
    bw: Option<Vec<i64>>,
}

impl RateLimiterConf {
    pub(crate) fn parse_token_bucket(args: &[i64]) -> TokenBucket {
        let mut bucket = TokenBucket::new(args[0], args[1]);
        if args.len() == 3 {
            bucket.set_one_time_burst(args[2]);
        }

        bucket
    }
    pub(crate) fn parse_rate_limiter(&self, dev: &mut dyn WithRateLimiterConf) {
        let mut rate_limiter = RateLimiter::new();
        if let Some(ops) = &self.ops {
            rate_limiter.set_ops(Self::parse_token_bucket(ops));
        }
        if let Some(bw) = &self.bw {
            rate_limiter.set_bandwidth(Self::parse_token_bucket(bw));
        }

        dev.set_rate_limiter(rate_limiter);
    }
}
