
use time::Timespec;

pub struct Entry {
    pub id: i32,
    pub amount: String,
    pub currency: String,
    pub ts: Timespec
}
