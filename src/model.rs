
use iron::typemap::Key;
use time::Timespec;

pub struct Entry {
    pub bank_account: String,
    pub amount: String,
    pub currency: String,
    pub id: i64,
    pub ts: Timespec
}

#[derive(Clone, Debug, RustcDecodable)]
pub struct Config {
    pub base_url: Option<String>,
    pub use_email: bool
}

impl Key for Config { type Value = Config; }
