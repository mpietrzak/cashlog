
use params::Map;
use time::Timespec;
use time::strptime;
use time;

pub fn parse_ts(s: &str) -> Result<Timespec, String> {
    let r = strptime(s, "%Y-%m-%d %H:%M:%S");
    match r {
        Ok(ref tm) => {
            let t = tm.to_timespec();
            Ok(t)
        }
        Err(ref e) => {
            Err(format!("{}", e))
        }
    }
}

pub fn parse_double(s: &str) -> Result<f64, String> {
    let r = s.parse::<f64>();
    match r {
        Ok(f) => {
            Ok(f)
        }
        _ => {
            Err(String::from("Failed to parse"))
        }
    }
}

pub fn get_ts(map: &Map, key: &str) -> Result<Timespec, String> {
    use params::Value;
    let v = map.get(key);
    match v {
        Some(&Value::String(ref ts_str)) => {
            parse_ts(ts_str)
        }
        _ => {
            Err(String::from("No such key"))
        }
    }
}

pub fn get_double(map: &Map, key: &str) -> Result<f64, String> {
    use params::Value;
    let v = map.get(key);
    match v {
        Some(&Value::String(ref double_str)) => {
            parse_double(double_str)
        }
        _ => {
            Err(String::from("No such key"))
        }
    }
}

pub fn get_str(map: &Map, key: &str) -> Result<String, String> {
    use params::Value;
    let v = map.get(key);
    match v {
        Some(&Value::String(ref s)) => {
            Ok(s.clone())
        }
        _ => {
            Err(String::from("No such key"))
        }
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
            warn!("Failed to format {:?} to \"{}\"",
                t,
                fmt);
            String::from("<invalid time>")
        }
    }
}

pub fn human_bytes(b: u64) -> String {
    let mut b = b;
    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    for unit in units.iter() {
        if b < 512 {
            return format!("{} {}", b, unit);
        } else {
            b = b / 1024;
        }
    }
    String::from("<out of range>")
}
