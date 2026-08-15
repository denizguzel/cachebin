use std::env;

use sysinfo::Disks;

use crate::models::PlatformInfo;
use crate::wsl;

pub fn info() -> Result<PlatformInfo, String> {
    let disks = Disks::new_with_refreshed_list();
    let home = env::var("USERPROFILE").unwrap_or_default();
    let home_drive = home
        .split('\\')
        .next()
        .map(|drive| format!("{drive}\\"))
        .unwrap_or_else(|| "C:\\".into());

    let disk = disks
        .iter()
        .find(|disk| disk.mount_point().to_string_lossy().eq_ignore_ascii_case(&home_drive))
        .or_else(|| disks.iter().max_by_key(|disk| disk.total_space()))
        .ok_or_else(|| "No disk information available".to_string())?;

    let total_bytes = disk.total_space();
    let free_bytes = disk.available_space();

    Ok(PlatformInfo {
        os_name: "Windows".into(),
        os_version: env::var("OS").unwrap_or_else(|_| "Windows_NT".into()),
        hostname: env::var("COMPUTERNAME").unwrap_or_default(),
        total_bytes,
        free_bytes,
        used_bytes: total_bytes.saturating_sub(free_bytes),
        wsl_distros: wsl::discover(),
    })
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;

    #[test]
    fn info_reports_windows_disk() {
        let info = info().expect("platform info should resolve on Windows");

        assert_eq!(info.os_name, "Windows");
        assert!(!info.hostname.is_empty());
        assert!(info.total_bytes > 0);
        assert!(info.free_bytes <= info.total_bytes);
        assert_eq!(info.used_bytes, info.total_bytes - info.free_bytes);
    }
}
