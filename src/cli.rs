use clap::Parser;
use crate::types::{SortBy, UsageThresholds};

#[derive(Parser, Debug)]
#[command(
    name = "disk-usage",
    version,
    about = "A disk usage visualization tool with better output than df -h",
    long_about = None
)]
pub struct Cli {
    /// Sort output by: usage, size, or mount
    #[arg(short, long, value_name = "FIELD", default_value = "usage")]
    pub sort: String,

    /// Minimum disk size to display in GB
    #[arg(short, long, value_name = "GB", default_value = "1")]
    pub min_size: u64,

    /// Show all disks (including those smaller than min-size)
    #[arg(short, long)]
    pub all: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Disable progress bars
    #[arg(long)]
    pub no_bars: bool,

    /// Set yellow threshold percentage
    #[arg(long, value_name = "PERCENT", default_value = "70")]
    pub yellow_threshold: f64,

    /// Set red threshold percentage
    #[arg(long, value_name = "PERCENT", default_value = "90")]
    pub red_threshold: f64,
}

impl Cli {
    pub fn parse_sort_by(&self) -> Result<SortBy, String> {
        match self.sort.to_lowercase().as_str() {
            "usage" | "u" => Ok(SortBy::Usage),
            "size" | "s" => Ok(SortBy::Size),
            "mount" | "m" | "mountpoint" => Ok(SortBy::MountPoint),
            _ => Err(format!(
                "Invalid sort option: '{}'. Use 'usage', 'size', or 'mount'",
                self.sort
            )),
        }
    }

    pub fn get_thresholds(&self) -> UsageThresholds {
        UsageThresholds {
            yellow: self.yellow_threshold,
            red: self.red_threshold,
        }
    }
}
