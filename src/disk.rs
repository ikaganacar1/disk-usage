use sysinfo::Disks;
use crate::types::DiskInfo;

pub struct DiskCollector {
    disks: Disks,
}

impl DiskCollector {
    pub fn new() -> Self {
        Self {
            disks: Disks::new_with_refreshed_list(),
        }
    }

    /// Collect all disk information
    pub fn collect_disks(&self) -> Vec<DiskInfo> {
        self.disks
            .iter()
            .map(|disk| {
                DiskInfo::new(
                    disk.name().to_string_lossy().to_string(),
                    disk.mount_point().to_path_buf(),
                    disk.total_space(),
                    disk.available_space(),
                    disk.file_system().to_string_lossy().to_string(),
                )
            })
            .collect()
    }

    /// Get filtered disks based on criteria
    pub fn get_filtered_disks(&self, min_gb: u64, include_all: bool) -> Vec<DiskInfo> {
        let mut disks = self.collect_disks();

        // Filter by size if not showing all
        if !include_all {
            disks.retain(|disk| disk.meets_size_threshold(min_gb));
        }

        disks
    }
}

impl Default for DiskCollector {
    fn default() -> Self {
        Self::new()
    }
}
