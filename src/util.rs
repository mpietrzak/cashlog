use chrono;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;

pub fn parse_ts(s: &str) -> Result<DateTime<Utc>, String> {
    chrono::Utc
        .datetime_from_str(s, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| format!("Error parsing datetime: {}", e))
}

/// Format timestamp using default format.
pub fn format_ts(dt: DateTime<Utc>) -> String {
    let fmt = "%Y-%m-%d %H:%M:%S";
    dt.format(fmt).to_string()
}

/*
pub fn human_bytes(b: u64) -> String {
    let mut f = b as f64;
    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    for unit in units.iter() {
        if f < 512.0 {
            return format!("{:.2} {}", f, unit);
        } else {
            f = f / 1024.0;
        }
    }
    panic!("Out of range");
}
*/
