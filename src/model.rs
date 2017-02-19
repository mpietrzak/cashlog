
use iron::typemap::Key;
use time::Timespec;

pub struct BankAccountInfo {
    pub bank_account: String,
    pub amount: String,
    pub currency: String,
    pub ts: Timespec
}

/// Details by currency.
pub struct CurrencyInfo {
    pub currency: String,
    pub amount: String,
    pub ts: Timespec
}

pub struct Entry {
    pub bank_account: String,
    pub amount: String,
    pub currency: String,
    pub id: i64,
    pub ts: Timespec
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub base_url: Option<String>,
    pub use_email: bool,
    pub port: Option<i32>
}

impl Key for Config { type Value = Config; }

/// Details about user account,
/// e.g. as displayed in profile page.
pub struct AccountInfo {
    pub created: Timespec,
    pub modified: Timespec,
    pub emails: Box<Vec<String>>
}
