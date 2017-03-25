
use std;
use std::fmt;
use std::error::Error;

use postgres;
use time::Timespec;

use model::AccountInfo;
use model::BankAccount;
use model::BankAccountInfo;
use model::CurrencyInfo;
use model::EntryInfo;

#[derive(Debug)]
pub struct DBError {
    desc: String,
}

impl DBError {
    fn new<X>(desc: X) -> DBError
        where X: ToString
    {
        DBError { desc: desc.to_string() }
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

impl std::convert::From<postgres::error::Error> for DBError {
    fn from(error: postgres::error::Error) -> DBError {
        DBError::new(format!("PostgreSQL Error: {}", error))
    }
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

pub fn set_session_value(conn: &mut postgres::Connection,
                         session_key: &str,
                         name: &str,
                         value: &str)
                         -> Result<(), DBError> {
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
        Err(e) => Err(DBError::new(&e.to_string())),
    }
}

pub fn delete_session(conn: &mut postgres::Connection, session_key: &str) -> Result<(), DBError> {
    match conn.execute("delete from session where key = $1", &[&session_key]) {
        Ok(_) => Ok(()),
        Err(e) => Err(DBError::new(&format!("Failed to delete session: {}", e))),
    }
}

pub fn get_account_id_by_email(conn: &mut postgres::Connection, email: &str) -> Result<Option<i64>, DBError> {
    let sql = "select account from account_email where email = $1";
    match conn.query(sql, &[&email]) {
        Ok(rows) => {
            match rows.iter().next() {
                Some(row) => {
                    match row.get_opt(0) {
                        None => Err(DBError::new("Invalid column")),
                        Some(res) => {
                            match res {
                                Err(e) => Err(DBError::new(&e.to_string())),
                                Ok(id) => Ok(Some(id)),
                            }
                        }
                    }
                }
                None => Ok(None),
            }
        }
        Err(e) => Err(DBError::new(&e.to_string())),
    }
}

pub fn create_account_with_email(conn: &mut postgres::Connection, email: &str) -> Result<i64, DBError> {
    let transaction = match conn.transaction() {
        Ok(t) => t,
        Err(e) => return Err(DBError::new(&e.to_string())),
    };
    let acc_id: i64 = match transaction.query("insert into account (
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
                Some(row) => {
                    match row.get_opt(0) {
                        None => return Err(DBError::new("Invalid column")),
                        Some(res) => {
                            match res {
                                Err(e) => return Err(DBError::new(e)),
                                Ok(id) => {
                                    debug!("create_account_with_email: new account id: {}", id);
                                    id
                                }
                            }
                        }
                    }
                }
                None => return Err(DBError::new("Insert did not return new id.")),
            }
        }
        Err(e) => return Err(DBError::new(&e.to_string())),
    };
    /// If we got here, then acc_id is the id of the new account.
    /// But we can't commit yet - we still need to add email.
    match transaction.execute("insert into account_email (
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
                Err(e) => Err(DBError::new(&e.to_string())),
            }
        }
        Err(e) => {
            // Insert failed...
            Err(DBError::new(&format!("Failed to insert account_email: {}", e)))
        }
    }
}

pub fn get_user_account_emails(conn: &mut postgres::Connection, acc_id: i64) -> Result<Box<Vec<String>>, DBError> {
    match conn.query("select email from account_email where account = $1",
                     &[&acc_id]) {
        Ok(rows) => {
            let v = rows.iter().map(|r| r.get(0)).collect();
            let b = Box::new(v);
            Ok(b)
        }
        Err(e) => Err(DBError::new(&format!("Error while getting account emails: {}", e))),
    }
}

pub fn get_user_account_info(conn: &mut postgres::Connection, acc_id: i64) -> Result<Option<AccountInfo>, DBError> {
    let emails = get_user_account_emails(conn, acc_id)?;
    match conn.query("select
            created,
            modified
        from account
        where id = $1",
                     &[&acc_id]) {
        Ok(rows) => {
            match rows.iter().next() {
                Some(row) => {
                    let created = row.get(0);
                    let modified = row.get(1);

                    let acc_info = AccountInfo {
                        created: created,
                        modified: modified,
                        emails: emails,
                    };
                    Ok(Some(acc_info))
                }
                None => Ok(None),
            }
        }
        Err(e) => Err(DBError::new(&format!("Error getting user account into: {}", e))),
    }
}

pub fn insert_login_token(conn: &mut postgres::Connection, account_id: &i64, token: &str) -> Result<(), DBError> {
    match conn.execute("insert into login_token (
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
        )",
                       &[&account_id, &token]) {
        Ok(_) => Ok(()),
        Err(e) => Err(DBError::new(&e.to_string())),
    }
}

/// Get account id by login token.
/// Only active tokens are queried.
pub fn get_login_token_account(conn: &mut postgres::Connection, token: &str) -> Result<Option<i64>, DBError> {
    match conn.query("select account
        from login_token
        where token = $1 and used = false",
                     &[&token]) {
        Err(e) => Err(DBError::new(&e.to_string())),
        Ok(rows) => {
            match rows.iter().next() {
                Some(row) => Ok(row.get(0)),
                None => Ok(None),
            }
        }
    }
}

pub fn insert_entry(conn: &mut postgres::Connection,
                    account_id: &i64,
                    bank_account: &i64,
                    ts: &Timespec,
                    amount_str: &str)
                    -> Result<(), DBError> {
    match conn.execute("insert into entry (
                            id,
                            bank_account,
                            ts,
                            amount,
                            deleted,
                            created,
                            modified
                        ) values (
                            nextval('entry_seq'),
                            (
                                select bank_account.id
                                from bank_account
                                where account = $1
                                and id = $2
                            ),                 -- bank account
                            $3,                -- ts
                            $4::text::numeric, -- amount
                            false,
                            current_timestamp,
                            current_timestamp
                        )",
                       &[&account_id, &bank_account, &ts, &amount_str]) {
        Ok(_) => Ok(()),
        Err(e) => Err(DBError::new(&format!("Failed to insert entry: {}", e))),
    }
}

/// Get CashLog entries for given account id.
/// Currently amount is String on Rust side - probably should be pair
/// of ints or something.
/// Returns only those entries that were not deleted (are not marked deleted).
pub fn get_entries(conn: &mut postgres::Connection, account_id: i64) -> Result<Vec<EntryInfo>, DBError> {
    let sql = "select
            entry.id,
            bank_account.name,
            entry.amount::text,
            bank_account.currency,
            entry.ts
        from
            entry
            join bank_account on (bank_account.id = entry.bank_account)
        where
            bank_account.account = $1
            and bank_account.deleted = false
            and entry.deleted = false
        order by entry.ts desc
        limit 1024";
    match conn.query(sql, &[&account_id]) {
        Ok(rows) => {
            Ok(rows.iter()
                   .map(|row| {
                EntryInfo {
                    id: row.get(0),
                    bank_account: row.get(1),
                    amount: row.get(2),
                    currency: row.get(3),
                    ts: row.get(4),
                }
            })
                   .collect())
        }
        Err(e) => Err(DBError::new(&e.to_string())),
    }
}

/// Delete entry.
pub fn delete_entry(conn: &mut postgres::Connection, account_id: i64, entry_id: i64) -> Result<(), DBError> {
    conn.execute("update entry set deleted = true, modified = current_timestamp
        where account = $1 and id = $2",
                 &[&account_id, &entry_id])?;
    Ok(())
}

pub fn get_bank_accounts(conn: &mut postgres::Connection, account_id: i64) -> Result<Vec<BankAccount>, DBError> {
    let sql = "
        select
            bank_account.id,
            bank_account.name,
            bank_account.currency
        from bank_account
        where
            bank_account.account = $1
            and bank_account.deleted = false
        order by bank_account.name, bank_account.id";
    match conn.query(sql, &[&account_id]) {
        Ok(rows) => {
            Ok(rows.iter()
                   .map(|row| {
                BankAccount {
                    id: row.get(0),
                    name: row.get(1),
                    currency: row.get(2),
                }
            })
                   .collect())
        }
        Err(err) => Err(DBError::from(err)),
    }
}

pub fn get_bank_account_infos(conn: &mut postgres::Connection,
                              account_id: i64)
                              -> Result<Vec<BankAccountInfo>, DBError> {
    let sql = "
        select
            bank_account_last_entry.name,
            last_entry.amount::text,
            bank_account_last_entry.currency,
            last_entry.ts
        from
            (
                select
                    bank_account.*,
                    (
                        select entry.id
                        from entry
                        where
                            entry.bank_account = bank_account.id
                            and entry.deleted = false
                        order by entry.ts desc
                        limit 1
                    ) as last_entry_id
                from bank_account
            ) as bank_account_last_entry
            left join entry as last_entry on (
                last_entry.id = bank_account_last_entry.last_entry_id
            )
        where
            bank_account_last_entry.account = $1
            and bank_account_last_entry.deleted = false
            and bank_account_last_entry.last_entry_id is not null
        order by
            bank_account_last_entry.name,
            bank_account_last_entry.currency
    ";
    match conn.query(sql, &[&account_id]) {
        Ok(rows) => {
            Ok(rows.iter()
                   .map(|row| {
                BankAccountInfo {
                    bank_account: row.get(0),
                    amount: row.get(1),
                    currency: row.get(2),
                    ts: row.get(3),
                }
            })
                   .collect())
        }
        Err(e) => Err(DBError::from(e)),
    }
}

pub fn insert_bank_account(conn: &mut postgres::Connection,
                           account_id: i64,
                           name: &str,
                           currency: &str)
                           -> Result<(), DBError> {
    let sql = "insert into bank_account (
        id,
        account,
        name,
        currency,
        created,
        modified
    ) values (
        nextval('bank_account_seq'),
        $1,
        $2,
        $3,
        current_timestamp,
        current_timestamp
    )";
    conn.execute(sql, &[&account_id, &name, &currency])?;
    Ok(())
}

pub fn get_currency_info(conn: &mut postgres::Connection, account_id: i64) -> Result<Vec<CurrencyInfo>, DBError> {
    let sql = "
        select
            -- Finally, group those selected entries to produce summaries.
            currency,
            sum(amount)::text,
            max(ts)
        from
            (
                select
                    bank_account.currency as currency,
                    (
                        select id
                        from entry
                        where
                            entry.bank_account = bank_account.id
                            and entry.deleted = false
                        order by ts desc
                        limit 1
                    ) as entry_id
                from
                    bank_account
                where
                    bank_account.account = $1
                    and bank_account.deleted = false
            ) as bank_account_last_entry_id
            join entry as last_entry on (last_entry.id = bank_account_last_entry_id.entry_id)
        group by currency
        order by currency";
    let rows = conn.query(sql, &[&account_id])?;
    Ok(rows.iter()
           .map(|row| {
        CurrencyInfo {
            currency: row.get(0),
            amount: row.get(1),
            ts: row.get(2),
        }
    })
           .collect())
}
