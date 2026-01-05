mod cli;
mod disk;
mod display;
mod filter;
mod types;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    // Parse CLI arguments
    let args = cli::Cli::parse();

    // Parse sort option
    let sort_by = args
        .parse_sort_by()
        .map_err(|e| anyhow::anyhow!(e))?;

    // Get thresholds
    let thresholds = args.get_thresholds();

    // Validate thresholds
    if args.yellow_threshold >= args.red_threshold {
        eprintln!("Warning: Yellow threshold should be less than red threshold");
    }

    // Collect disk information
    let collector = disk::DiskCollector::new();
    let mut disks = collector.get_filtered_disks(args.min_size, args.all);

    // Handle case when no disks are found
    if disks.is_empty() && !args.all {
        eprintln!(
            "No disks found with size >= {} GB. Try --all to show all disks.",
            args.min_size
        );
        std::process::exit(0);
    }

    // Sort disks
    filter::sort_disks(&mut disks, sort_by);

    // Display results
    display::display_disks(&disks, &thresholds, !args.no_color, !args.no_bars);

    Ok(())
}
