
use params::Map;
use time;
use time::Timespec;
use time::strptime;

pub fn parse_ts(s: &str) -> Result<Timespec, String> {
    let r = strptime(s, "%Y-%m-%d %H:%M:%S");
    match r {
        Ok(ref tm) => {
            let t = tm.to_timespec();
            Ok(t)
        }
        Err(ref e) => Err(format!("{}", e)),
    }
}

pub fn parse_double(s: &str) -> Result<f64, String> {
    let r = s.parse::<f64>();
    match r {
        Ok(f) => Ok(f),
        _ => Err(String::from("Failed to parse")),
    }
}

pub fn get_ts(map: &Map, key: &str) -> Result<Timespec, String> {
    use params::Value;
    let v = map.get(key);
    match v {
        Some(&Value::String(ref ts_str)) => parse_ts(ts_str),
        _ => Err(String::from("No such key")),
    }
}

pub fn get_double(map: &Map, key: &str) -> Result<f64, String> {
    use params::Value;
    let v = map.get(key);
    match v {
        Some(&Value::String(ref double_str)) => parse_double(double_str),
        _ => Err(String::from("No such key")),
    }
}

pub fn get_i64(map: &Map, key: &str) -> Result<i64, String> {
    use params::Value;
    match map.get(key) {
        Some(&Value::String(ref s)) => match s.parse() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        },
        _ => Err(format!("No such key: \"{}\"", key)),
    }
}

pub fn get_str(map: &Map, key: &str) -> Result<String, String> {
    use params::Value;
    let v = map.get(key);
    match v {
        Some(&Value::String(ref s)) => Ok(s.clone()),
        _ => Err(format!("No such key: \"{}\"", key)),
    }
}

pub fn format_ts(t: time::Timespec) -> String {
    let tm = time::at_utc(t);
    let fmt = "%Y-%m-%d %H:%M:%S";
    match time::strftime(fmt, &tm) {
        Ok(s) => s,
        Err(_) => {
            // Both fmt and t should be safe correct values here,
            // so we warn if something is not right.
            warn!("Failed to format {:?} to \"{}\"", t, fmt);
            String::from("<invalid time>")
        }
    }
}

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
    String::from("<out of range>")
}
