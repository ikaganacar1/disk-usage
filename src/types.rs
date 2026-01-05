use std::path::PathBuf;

/// Represents a disk partition with usage information
#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: PathBuf,
    pub total_space: u64,      // in bytes
    pub available_space: u64,  // in bytes
    pub used_space: u64,       // in bytes
    pub usage_percentage: f64, // 0.0 to 100.0
    pub filesystem_type: String,
}

impl DiskInfo {
    pub fn new(
        name: String,
        mount_point: PathBuf,
        total_space: u64,
        available_space: u64,
        filesystem_type: String,
    ) -> Self {
        let used_space = total_space.saturating_sub(available_space);
        let usage_percentage = if total_space > 0 {
            (used_space as f64 / total_space as f64) * 100.0
        } else {
            0.0
        };

        Self {
            name,
            mount_point,
            total_space,
            available_space,
            used_space,
            usage_percentage,
            filesystem_type,
        }
    }

    /// Returns true if disk is >= threshold GB
    pub fn meets_size_threshold(&self, min_gb: u64) -> bool {
        let min_bytes = min_gb * 1024 * 1024 * 1024;
        self.total_space >= min_bytes
    }
}

/// Sorting options for disk list
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    Usage,
    Size,
    MountPoint,
}

/// Color thresholds for usage percentage
#[derive(Debug, Clone, Copy)]
pub struct UsageThresholds {
    pub yellow: f64,  // Default: 70.0
    pub red: f64,     // Default: 90.0
}

impl Default for UsageThresholds {
    fn default() -> Self {
        Self {
            yellow: 70.0,
            red: 90.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_info_creation() {
        let disk = DiskInfo::new(
            "sda1".to_string(),
            PathBuf::from("/"),
            1024 * 1024 * 1024 * 100, // 100 GB
            1024 * 1024 * 1024 * 30,  // 30 GB free
            "ext4".to_string(),
        );
        assert_eq!(disk.used_space, 1024 * 1024 * 1024 * 70);
        assert_eq!(disk.usage_percentage, 70.0);
        assert!(disk.meets_size_threshold(1));
        assert!(!disk.meets_size_threshold(200));
    }

    #[test]
    fn test_size_threshold() {
        let disk = DiskInfo::new(
            "sda1".to_string(),
            PathBuf::from("/"),
            500 * 1024 * 1024,  // 500 MB
            100 * 1024 * 1024,  // 100 MB free
            "ext4".to_string(),
        );
        assert!(!disk.meets_size_threshold(1));  // Less than 1 GB
    }
}
