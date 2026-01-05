use crate::types::{DiskInfo, SortBy};

pub fn sort_disks(disks: &mut [DiskInfo], sort_by: SortBy) {
    match sort_by {
        SortBy::Usage => {
            disks.sort_by(|a, b| {
                b.usage_percentage
                    .partial_cmp(&a.usage_percentage)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        SortBy::Size => {
            disks.sort_by(|a, b| b.total_space.cmp(&a.total_space));
        }
        SortBy::MountPoint => {
            disks.sort_by(|a, b| a.mount_point.cmp(&b.mount_point));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_sort_by_usage() {
        let mut disks = vec![
            DiskInfo::new(
                "sda1".to_string(),
                PathBuf::from("/"),
                100 * 1024 * 1024 * 1024,
                50 * 1024 * 1024 * 1024,
                "ext4".to_string(),
            ),
            DiskInfo::new(
                "sdb1".to_string(),
                PathBuf::from("/home"),
                100 * 1024 * 1024 * 1024,
                10 * 1024 * 1024 * 1024,
                "ext4".to_string(),
            ),
        ];

        sort_disks(&mut disks, SortBy::Usage);
        assert!(disks[0].usage_percentage > disks[1].usage_percentage);
    }

    #[test]
    fn test_sort_by_size() {
        let mut disks = vec![
            DiskInfo::new(
                "sda1".to_string(),
                PathBuf::from("/"),
                50 * 1024 * 1024 * 1024,
                10 * 1024 * 1024 * 1024,
                "ext4".to_string(),
            ),
            DiskInfo::new(
                "sdb1".to_string(),
                PathBuf::from("/home"),
                100 * 1024 * 1024 * 1024,
                10 * 1024 * 1024 * 1024,
                "ext4".to_string(),
            ),
        ];

        sort_disks(&mut disks, SortBy::Size);
        assert!(disks[0].total_space > disks[1].total_space);
    }
}
