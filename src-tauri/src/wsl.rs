use std::process::Command;

use crate::models::WslDistro;

const WSL_EXT: &str = "wsl.exe";

pub fn discover() -> Vec<WslDistro> {
    let output = match Command::new(WSL_EXT).args(["-l", "-v"]).output() {
        Ok(output) if output.status.success() => output,
        _ => return Vec::new(),
    };

    let stdout = decode_wsl_output(&output.stdout);
    parse_distros(&stdout)
}

pub fn home_dirs(distro: &WslDistro) -> Vec<String> {
    merge_homes(query_home(&distro.name))
}

fn merge_homes(home: Option<String>) -> Vec<String> {
    let mut homes = Vec::new();

    if let Some(home) = home {
        if !homes.contains(&home) {
            homes.push(home);
        }
    }

    if !homes.contains(&"/root".to_string()) {
        homes.push("/root".into());
    }

    homes
}

fn query_home(distro: &str) -> Option<String> {
    let output = Command::new(WSL_EXT)
        .args(["-d", distro, "-e", "sh", "-c", "printf '%s' \"$HOME\""])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let home = decode_wsl_output(&output.stdout).trim().to_string();
    if home.is_empty() {
        None
    } else {
        Some(home)
    }
}

fn parse_distros(stdout: &str) -> Vec<WslDistro> {
    stdout.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<WslDistro> {
    let trimmed = line.trim().trim_start_matches('\u{feff}');
    if trimmed.is_empty() || trimmed.contains("Windows Subsystem") || trimmed.contains("WSL") {
        return None;
    }

    let mut tokens = trimmed.split_whitespace().collect::<Vec<_>>();
    if tokens.first() == Some(&"*") {
        tokens.remove(0);
    }

    if tokens.len() < 2 || tokens[0] == "NAME" {
        return None;
    }

    let name = tokens[0].to_string();
    let state = tokens[1].to_string();
    let version = tokens
        .get(2)
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(2);

    Some(WslDistro {
        name: name.clone(),
        state,
        version,
        root: format!(r"\\wsl.localhost\{name}"),
    })
}

fn decode_wsl_output(bytes: &[u8]) -> String {
    let has_bom = bytes.starts_with(&[0xFF, 0xFE]);
    if has_bom || looks_like_utf16le(bytes) {
        let offset = if has_bom { 2 } else { 0 };
        let units = bytes[offset..]
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<_>>();
        return String::from_utf16_lossy(&units);
    }

    String::from_utf8_lossy(bytes).trim_start_matches('\u{feff}').to_string()
}

fn looks_like_utf16le(bytes: &[u8]) -> bool {
    if bytes.len() < 4 || bytes.len() % 2 != 0 {
        return false;
    }

    // UTF-16LE ASCII text carries a NUL high byte (odd index) per code unit.
    let units = bytes.len() / 2;
    let nul_high_bytes = bytes[1..].iter().step_by(2).filter(|byte| **byte == 0).count();
    nul_high_bytes * 10 >= units * 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_distro_lines() {
        let stdout = "\u{feff}  NAME      STATE           VERSION\r\n* Ubuntu    Running         2\r\n  Debian    Stopped         1\r\n";
        let distros = parse_distros(stdout);

        assert_eq!(distros.len(), 2);
        assert_eq!(distros[0].name, "Ubuntu");
        assert_eq!(distros[0].state, "Running");
        assert_eq!(distros[0].version, 2);
        assert_eq!(distros[0].root, r"\\wsl.localhost\Ubuntu");
        assert_eq!(distros[1].name, "Debian");
        assert_eq!(distros[1].version, 1);
    }

    #[test]
    fn skips_header_and_note_lines() {
        let stdout = "Windows Subsystem for Linux Distributions:\nNAME STATE VERSION\n";
        assert!(parse_distros(stdout).is_empty());
    }

    #[test]
    fn parse_line_strips_default_marker() {
        let distro = parse_line("* Ubuntu    Running         2").expect("distro");
        assert_eq!(distro.name, "Ubuntu");
        assert_eq!(distro.state, "Running");
        assert_eq!(distro.version, 2);
    }

    #[test]
    fn parse_line_defaults_version_when_missing() {
        let distro = parse_line("Ubuntu    Running").expect("distro");
        assert_eq!(distro.name, "Ubuntu");
        assert_eq!(distro.version, 2);
    }

    #[test]
    fn parse_line_rejects_garbage() {
        assert!(parse_line("NAME      STATE           VERSION").is_none());
        assert!(parse_line("").is_none());
        assert!(parse_line("Windows Subsystem for Linux Distributions:").is_none());
    }

    #[test]
    fn merge_homes_dedups_and_appends_root() {
        assert_eq!(
            merge_homes(Some("/home/dev".into())),
            vec!["/home/dev".to_string(), "/root".to_string()]
        );
        assert_eq!(merge_homes(Some("/root".into())), vec!["/root".to_string()]);
        assert_eq!(merge_homes(None), vec!["/root".to_string()]);
    }

    #[test]
    fn decodes_utf16le_output() {
        let mut bytes = vec![0xFF, 0xFE];
        bytes.extend_from_slice(&[0x55, 0x00, 0x62, 0x00]);
        assert_eq!(decode_wsl_output(&bytes), "Ub");
    }

    #[test]
    fn decodes_utf16le_without_bom() {
        let bytes = b"N\x00A\x00M\x00E\x00";
        assert_eq!(decode_wsl_output(bytes), "NAME");
    }

    #[test]
    fn decodes_utf8_output() {
        assert_eq!(decode_wsl_output(b"Ubuntu"), "Ubuntu");
    }

    #[test]
    fn parses_bomless_utf16le_wsl_table() {
        let table = "  NAME      STATE           VERSION\r\n* Ubuntu    Running         2\r\n";
        let bytes = table
            .encode_utf16()
            .flat_map(|unit| unit.to_le_bytes())
            .collect::<Vec<_>>();

        let stdout = decode_wsl_output(&bytes);
        let distros = parse_distros(&stdout);

        assert_eq!(distros.len(), 1);
        assert_eq!(distros[0].name, "Ubuntu");
        assert_eq!(distros[0].state, "Running");
        assert_eq!(distros[0].version, 2);
        assert_eq!(distros[0].root, r"\\wsl.localhost\Ubuntu");
    }
}
