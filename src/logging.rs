//! Helpers and utils related to logging.

use std::env;
use std::io::Write;

use env_logger::Builder;
use log::LevelFilter;

/// Does not return error, instead just panics on error,
/// since logging is pretty essential.
pub fn env_logger_init() {
    let format = |buf: &mut env_logger::fmt::Formatter, record: &log::Record| {
        let ts = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(
            buf,
            "{} {} {} {}",
            ts,
            record.level(),
            record.module_path().unwrap_or(""),
            record.args()
        )
    };
    let mut builder = Builder::new();
    builder.format(format).filter(None, LevelFilter::Debug);
    if env::var("RUST_LOG").is_ok() {
        builder.parse_env(&env::var("RUST_LOG").unwrap());
    }
    builder.init();
}
