use chrono::DateTime;
use chrono::Utc;

/// Just a Bank Account.
pub struct BankAccount {
    pub id: i64,
    pub name: String,
    pub currency: String,
}

/// Bank Account with some other joins.
pub struct BankAccountInfo {
    pub bank_account: String,
    pub amount: String,
    pub currency: String,
    pub ts: DateTime<Utc>,
}

/// Details by currency.
pub struct CurrencyInfo {
    pub currency: String,
    pub amount: String,
    pub ts: DateTime<Utc>,
}

/// Bank account balance log entry.
pub struct EntryInfo {
    pub amount: String,
    pub bank_account: String,
    pub currency: String,
    pub id: i64,
    pub ts: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub use_email: bool,
    pub port: Option<i32>,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
}

/// Details about user account,
/// e.g. as displayed in profile page.
pub struct AccountInfo {
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub emails: Box<Vec<String>>,
}
