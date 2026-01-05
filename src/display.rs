use colored::*;
use crate::types::{DiskInfo, UsageThresholds};

/// Format bytes to human-readable size
pub fn format_bytes(bytes: u64) -> String {
    const TB: u64 = 1024 * 1024 * 1024 * 1024;
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    }
}

/// Get color based on usage percentage and thresholds
pub fn get_usage_color(usage: f64, thresholds: &UsageThresholds, enable_color: bool) -> Color {
    if !enable_color {
        return Color::White;
    }

    if usage >= thresholds.red {
        Color::Red
    } else if usage >= thresholds.yellow {
        Color::Yellow
    } else {
        Color::Green
    }
}

/// Create a progress bar for disk usage
pub fn create_progress_bar(
    disk: &DiskInfo,
    thresholds: &UsageThresholds,
    enable_bars: bool,
    enable_color: bool,
) -> String {
    if !enable_bars {
        return format!("{:.1}%", disk.usage_percentage);
    }

    let bar_width: usize = 20;
    let filled = ((disk.usage_percentage / 100.0) * bar_width as f64).round() as usize;
    let empty = bar_width.saturating_sub(filled);

    let bar_char = "█";
    let empty_char = "░";

    let bar_filled = bar_char.repeat(filled);
    let bar_empty = empty_char.repeat(empty);

    let color = get_usage_color(disk.usage_percentage, thresholds, enable_color);

    if enable_color {
        format!(
            "{} {:.1}%",
            format!("{}{}", bar_filled, bar_empty).color(color),
            disk.usage_percentage
        )
    } else {
        format!("{}{} {:.1}%", bar_filled, bar_empty, disk.usage_percentage)
    }
}

/// Display disk information in a formatted table
pub fn display_disks(
    disks: &[DiskInfo],
    thresholds: &UsageThresholds,
    enable_color: bool,
    enable_bars: bool,
) {
    if disks.is_empty() {
        println!("No disks found matching the criteria.");
        return;
    }

    // Header
    let header = format!(
        "{:<20} {:<30} {:<12} {:<12} {:<12} {:<30}",
        "Filesystem", "Mounted on", "Size", "Used", "Available", "Use%"
    );

    if enable_color {
        println!("{}", header.bold().underline());
    } else {
        println!("{}", header);
        println!("{}", "-".repeat(header.len()));
    }

    // Rows
    for disk in disks {
        let filesystem = truncate_string(&disk.name, 20);
        let mount_point = truncate_string(&disk.mount_point.display().to_string(), 30);
        let size = format_bytes(disk.total_space);
        let used = format_bytes(disk.used_space);
        let available = format_bytes(disk.available_space);
        let usage_bar = create_progress_bar(disk, thresholds, enable_bars, enable_color);

        let color = get_usage_color(disk.usage_percentage, thresholds, enable_color);

        let row = format!(
            "{:<20} {:<30} {:<12} {:<12} {:<12} {}",
            filesystem, mount_point, size, used, available, usage_bar
        );

        if enable_color {
            println!("{}", row.color(color));
        } else {
            println!("{}", row);
        }
    }

    // Summary
    println!();
    let total_size: u64 = disks.iter().map(|d| d.total_space).sum();
    let total_used: u64 = disks.iter().map(|d| d.used_space).sum();
    let total_available: u64 = disks.iter().map(|d| d.available_space).sum();

    let summary = format!(
        "Total: {} size, {} used, {} available",
        format_bytes(total_size),
        format_bytes(total_used),
        format_bytes(total_available)
    );

    if enable_color {
        println!("{}", summary.bold());
    } else {
        println!("{}", summary);
    }
}

/// Truncate string to max length with ellipsis
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
        assert_eq!(format_bytes(1024u64 * 1024 * 1024 * 1024), "1.00 TB");
    }

    #[test]
    fn test_usage_color() {
        let thresholds = UsageThresholds::default();
        assert_eq!(get_usage_color(50.0, &thresholds, true), Color::Green);
        assert_eq!(get_usage_color(75.0, &thresholds, true), Color::Yellow);
        assert_eq!(get_usage_color(95.0, &thresholds, true), Color::Red);
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello", 10), "hello");
        assert_eq!(truncate_string("hello world this is long", 10), "hello w...");
    }
}
