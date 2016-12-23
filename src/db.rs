
use std::fmt;
use std::error::Error;

use postgres;
use time::Timespec;

use model::AccountInfo;
use model::Entry;

#[derive(Debug)]
pub struct DBError {
    desc: String
}

impl DBError {
    fn new(desc: &str) -> DBError {
        DBError {
            desc: String::from(desc)
        }
    }
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBError: {}", self.desc)
    }
}

impl Error for DBError {
    fn description(&self) -> &str {
        &self.desc
    }
}

/// Connect to DB.
/// TODO: Should return Error.
pub fn connect() -> postgres::Connection {
    postgres::Connection::connect(
        "postgres://cashlog@localhost/cashlog",
        postgres::TlsMode::None
    ).unwrap()
}

pub fn get_session_value(conn: &mut postgres::Connection, session_key: &str, name: &str) -> Option<String> {
    let sql = "select value from session where key = $1 and name = $2";
    let r_rows = conn.query(sql, &[&session_key, &name]);
    match r_rows {
        Ok(rows) => {
            // There should be max one row.
            if let Some(row) = rows.iter().next() {
                let value: String = row.get(0);
                Some(value)
            } else {
                // That's ok, it just does not exist.
                None
            }

        }
        Err(e) => {
            warn!("Failed to get session value: {}", e);
            None
        }
    }
}

pub fn set_session_value(conn: &mut postgres::Connection, session_key: &str, name: &str, value: &str) -> Result<(), DBError> {
    let sql = "insert into session (
            id,
            key,
            name,
            value,
            created,
            modified
        ) values (
            nextval('session_seq'),
            $1,
            $2,
            $3,
            current_timestamp,
            current_timestamp
        )
        on conflict (key, name)
        do update set
            value = $3,
            modified = current_timestamp";
    match conn.execute(sql, &[&session_key, &name, &value]) {
        Ok(_) => Ok(()),
        Err(e) => Err(DBError::new(&e.to_string()))
    }
}

pub fn get_account_id_by_email(
        conn: &mut postgres::Connection,
        email: &str) -> Result<Option<i64>, DBError> {
    let sql = "select account from account_email where email = $1";
    match conn.query(sql, &[&email]) {
        Ok(rows) => {
            match rows.iter().next() {
                Some(row) => {
                    match row.get_opt(0) {
                        None => Err(DBError::new("Invalid column")),
                        Some(res) => match res {
                            Err(e) => Err(DBError::new(&e.to_string())),
                            Ok(id) => Ok(Some(id))
                        }
                    }
                }
                None => Ok(None)
            }
        }
        Err(e) => {
            Err(DBError::new(&e.to_string()))
        }
    }
}

pub fn create_account_with_email(
        conn: &mut postgres::Connection,
        email: &str) -> Result<i64, DBError> {
    let transaction = match conn.transaction() {
        Ok(t) => t,
        Err(e) => return Err(DBError::new(&e.to_string()))
    };
    let acc_id: i64 = match transaction.query(
        "insert into account (
            id,
            created,
            modified
        ) values (
            nextval('account_seq'),
            current_timestamp,
            current_timestamp
        )
        returning id",
        &[]) {
        Ok(rows) => {
            match rows.iter().next() {
                Some(row) => match row.get_opt(0) {
                    None => return Err(DBError::new("Invalid column")),
                    Some(res) => match res {
                        Err(e) => return Err(DBError::new(&e.to_string())),
                        Ok(id) => {
                            debug!("create_account_with_email: new account id: {}", id);
                            id
                        }
                    }
                },
                None => return Err(DBError::new("Insert did not return new id."))
            }
        }
        Err(e) => return Err(DBError::new(&e.to_string()))
    };
    /// If we got here, then acc_id is the id of the new account.
    /// But we can't commit yet - we still need to add email.
    match transaction.execute(
        "insert into account_email (
            id,
            account,
            email,
            created,
            modified
        ) values (
            nextval('account_email_seq'),
            $1,
            $2,
            current_timestamp,
            current_timestamp
        )",
        &[&acc_id, &email]) {
        Ok(_) => {
            /// Insert ok, let's commit...
            match transaction.commit() {
                Ok(_) => {
                    debug!("create_account_with_email: commit done");
                    Ok(acc_id)
                }
                Err(e) => Err(DBError::new(&e.to_string()))
            }
        }
        Err(e) => {
            // Insert failed...
            Err(DBError::new(&format!("Failed to insert account_email: {}", e)))
        }
    }
}

pub fn get_user_account_info(conn: &mut postgres::Connection, acc_id: i64) -> Result<Option<AccountInfo>, DBError> {
    match conn.query(
        "select
            created,
            modified
        from account
        where id = $1",
        &[&acc_id]) {
        Ok(rows) => match rows.iter().next() {
            Some(row) => {
                let created = row.get(0);
                let modified = row.get(1);
                let acc_info = AccountInfo {
                    created: created,
                    modified: modified,
                    emails: Box::new(Vec::new())
                };
                Ok(Some(acc_info))
            }
            None => Ok(None)
        },
        Err(e) => Err(DBError::new(&format!("Error getting user account into: {}", e)))
    }
}

pub fn insert_login_token(
        conn: &mut postgres::Connection,
        account_id: &i64,
        token: &str) -> Result<(), DBError> {
    match conn.execute(
        "insert into login_token (
            id,
            account,
            token,
            used,
            used_ts,
            created,
            modified
        ) values (
            nextval('login_token_seq'),
            $1,
            $2,
            false,
            null,
            current_timestamp,
            current_timestamp
        )", &[&account_id, &token]) {
        Ok(_) => Ok(()),
        Err(e) => Err(DBError::new(&e.to_string()))
    }
}

/// Get account id by login token.
/// Only active tokens are queried.
pub fn get_login_token_account(
        conn: &mut postgres::Connection,
        token: &str) -> Result<Option<i64>, DBError> {
    match conn.query(
        "select account
        from login_token
        where token = $1 and used = false",
        &[&token]) {
        Err(e) => Err(DBError::new(&e.to_string())),
        Ok(rows) => match rows.iter().next() {
            Some(row) => Ok(row.get(0)),
            None => Ok(None)
        }
    }
}

pub fn insert_entry(
        conn: &mut postgres::Connection,
        account: i64,
        bank_account: &str,
        ts: &Timespec,
        amount_str: &str,
        currency: &str) -> Result<(), DBError> {
    match conn.execute(
        "insert into entry (
            id,
            account,
            bank_account,
            ts,
            amount,
            currency
        ) values (
            nextval('entry_seq'),
            $1,                -- account
            $2,                -- bank_account
            $3,                -- ts
            $4::text::numeric, -- amount
            $5                 -- currency
        )",
        &[&account, &bank_account, &ts, &amount_str, &currency]) {
        Ok(_) => Ok(()),
        Err(e) => Err(DBError::new(&format!("Failed to insert entry: {}", e)))
    }
}

/// Get CashLog entries for given account id.
/// Currently amount is String on Rust side - probably should be pair
/// of ints or something.
pub fn get_entries(conn: &mut postgres::Connection, account_id: i64) -> Result<Vec<Entry>, DBError> {
    let sql = "select
            id,
            bank_account,
            amount::text,
            currency,
            ts
        from entry
        where account = $1
        order by ts desc";
    match conn.query(sql, &[&account_id]) {
        Ok(rows) => {
            Ok(rows.iter().map(
                    |row| Entry {
                        id: row.get(0),
                        bank_account: row.get(1),
                        amount: row.get(2),
                        currency: row.get(3),
                        ts: row.get(4)
                    }
                ).collect())
        }
        Err(e) => {
            Err(DBError::new(&e.to_string()))
        }
    }
}
