use std::time::SystemTime;

/// Formats a [`SystemTime`] as a display-friendly local timestamp.
pub fn format_time(time: SystemTime) -> String {
    let datetime: chrono::DateTime<chrono::Local> = time.into();
    datetime.format("%Y-%m-%d %H:%M").to_string()
}

/// Returns the current local time as an RFC 3339 string for persistence.
pub fn now_rfc3339() -> String {
    chrono::Local::now().to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_time_is_non_empty() {
        assert!(!format_time(SystemTime::now()).is_empty());
    }

    #[test]
    fn now_rfc3339_is_parseable() {
        let value = now_rfc3339();
        assert!(value.contains('T'));
        assert!(chrono::DateTime::parse_from_rfc3339(&value).is_ok());
    }
}
