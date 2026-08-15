use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Safe,
    Caution,
    Risky,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    pub kind: String,
    pub distro: Option<String>,
}

impl Environment {
    pub fn windows() -> Self {
        Self {
            kind: "windows".into(),
            distro: None,
        }
    }

    pub fn wsl(distro: impl Into<String>) -> Self {
        Self {
            kind: "wsl".into(),
            distro: Some(distro.into()),
        }
    }

    pub fn id(&self) -> String {
        match self.kind.as_str() {
            "wsl" => format!("wsl-{}", self.distro.as_deref().unwrap_or("unknown")),
            _ => "windows".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheEntry {
    pub id: String,
    pub category: String,
    pub name: String,
    pub path: String,
    pub environment: Environment,
    pub size_bytes: u64,
    pub risk: RiskLevel,
    pub description: String,
    pub last_modified: Option<String>,
    pub rebuildable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub entries: Vec<CacheEntry>,
    pub total_bytes: u64,
    pub location_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WslDistro {
    pub name: String,
    pub state: String,
    pub version: u32,
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformInfo {
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub used_bytes: u64,
    pub wsl_distros: Vec<WslDistro>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectArtifact {
    pub id: String,
    pub project_path: String,
    pub name: String,
    pub path: String,
    pub environment: Environment,
    pub size_bytes: u64,
    pub risk: RiskLevel,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargeFile {
    pub id: String,
    pub name: String,
    pub path: String,
    pub environment: Environment,
    pub size_bytes: u64,
    pub file_type: String,
    pub last_modified: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn risk_level_serializes_lowercase() {
        assert_eq!(serde_json::to_string(&RiskLevel::Safe).unwrap(), r#""safe""#);
        assert_eq!(serde_json::to_string(&RiskLevel::Caution).unwrap(), r#""caution""#);
        assert_eq!(serde_json::to_string(&RiskLevel::Risky).unwrap(), r#""risky""#);
        assert_eq!(serde_json::from_str::<RiskLevel>(r#""safe""#).unwrap(), RiskLevel::Safe);
        assert_eq!(serde_json::from_str::<RiskLevel>(r#""risky""#).unwrap(), RiskLevel::Risky);
    }

    #[test]
    fn risk_level_rejects_unknown_value() {
        assert!(serde_json::from_str::<RiskLevel>(r#""unknown""#).is_err());
    }

    #[test]
    fn environment_kind_and_id() {
        let windows = Environment::windows();
        assert_eq!(windows.kind, "windows");
        assert!(windows.distro.is_none());
        assert_eq!(windows.id(), "windows");

        let wsl = Environment::wsl("Ubuntu");
        assert_eq!(wsl.kind, "wsl");
        assert_eq!(wsl.distro.as_deref(), Some("Ubuntu"));
        assert_eq!(wsl.id(), "wsl-Ubuntu");
    }

    #[test]
    fn cache_entry_round_trips_with_camel_case_keys() {
        let entry = CacheEntry {
            id: "windows-npm-cache".into(),
            category: "Node.js".into(),
            name: "npm cache".into(),
            path: r"C:\cache\npm".into(),
            environment: Environment::windows(),
            size_bytes: 1024,
            risk: RiskLevel::Safe,
            description: "Downloaded npm package tarballs".into(),
            last_modified: Some("2026-08-14 09:12".into()),
            rebuildable: true,
        };

        let json = serde_json::to_value(&entry).unwrap();
        assert_eq!(json["sizeBytes"], 1024);
        assert_eq!(json["lastModified"], "2026-08-14 09:12");
        assert_eq!(json["environment"]["kind"], "windows");

        let back: CacheEntry = serde_json::from_value(json).unwrap();
        assert_eq!(back.id, entry.id);
        assert_eq!(back.size_bytes, 1024);
    }

    #[test]
    fn scan_result_round_trips_with_camel_case_keys() {
        let result = ScanResult {
            entries: vec![],
            total_bytes: 2048,
            location_count: 3,
        };

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["totalBytes"], 2048);
        assert_eq!(json["locationCount"], 3);

        let back: ScanResult = serde_json::from_value(json).unwrap();
        assert_eq!(back.total_bytes, 2048);
        assert_eq!(back.location_count, 3);
    }

    #[test]
    fn platform_info_round_trips_with_camel_case_keys() {
        let info = PlatformInfo {
            os_name: "Windows".into(),
            os_version: "Windows_NT".into(),
            hostname: "DESKTOP".into(),
            total_bytes: 100,
            free_bytes: 20,
            used_bytes: 80,
            wsl_distros: vec![WslDistro {
                name: "Ubuntu".into(),
                state: "Running".into(),
                version: 2,
                root: r"\\wsl.localhost\Ubuntu".into(),
            }],
        };

        let json = serde_json::to_value(&info).unwrap();
        assert_eq!(json["osName"], "Windows");
        assert_eq!(json["wslDistros"][0]["name"], "Ubuntu");
        assert_eq!(json["wslDistros"][0]["version"], 2);

        let back: PlatformInfo = serde_json::from_value(json).unwrap();
        assert_eq!(back.wsl_distros[0].root, r"\\wsl.localhost\Ubuntu");
    }
}
