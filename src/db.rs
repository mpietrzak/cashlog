use std;
use std::error::Error;
use std::fmt;

use chrono::TimeZone;
use postgres;

use crate::model::AccountInfo;
use crate::model::BankAccount;
use crate::model::BankAccountInfo;
use crate::model::CurrencyInfo;
use crate::model::EntryInfo;

#[derive(Debug)]
pub struct DbError {
    desc: String,
}

impl DbError {
    fn new<X>(desc: X) -> DbError
    where
        X: ToString,
    {
        DbError {
            desc: desc.to_string(),
        }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DbError: {}", self.desc)
    }
}

impl Error for DbError {
    fn description(&self) -> &str {
        &self.desc
    }
}

impl std::convert::From<postgres::error::Error> for DbError {
    fn from(error: postgres::error::Error) -> DbError {
        DbError::new(format!("PostgreSQL Error: {}", error))
    }
}

pub fn get_sess_val(conn: &mut postgres::Client, sess_key: &str, name: &str) -> Option<String> {
    let sql = "select value from session where key = $1 and name = $2";
    match conn.query(sql, &[&sess_key, &name]) {
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

pub fn set_session_value(
    conn: &mut postgres::Client,
    session_key: &str,
    name: &str,
    value: &str,
) -> Result<(), DbError> {
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
        Err(e) => Err(DbError::new(&e.to_string())),
    }
}

pub fn delete_session(conn: &mut postgres::Client, session_key: &str) -> Result<(), DbError> {
    match conn.execute("delete from session where key = $1", &[&session_key]) {
        Ok(_) => Ok(()),
        Err(e) => Err(DbError::new(&format!("Failed to delete session: {}", e))),
    }
}

pub fn get_acc_id_by_email(
    conn: &mut postgres::Client,
    email: &str,
) -> Result<Option<i64>, DbError> {
    let sql = "select account from account_email where email = $1";
    match conn.query(sql, &[&email]) {
        Ok(rows) => match rows.iter().next() {
            Some(row) => match row.get(0) {
                None => Err(DbError::new("Invalid column")),
                Some(acc_id) => Ok(acc_id),
            },
            None => Ok(None),
        },
        Err(e) => Err(DbError::new(&e.to_string())),
    }
}

pub fn create_acc_with_email(conn: &mut postgres::Client, email: &str) -> Result<i64, DbError> {
    let mut transaction = match conn.transaction() {
        Ok(t) => t,
        Err(e) => return Err(DbError::new(&e.to_string())),
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
        &[],
    ) {
        Ok(rows) => match rows.iter().next() {
            Some(row) => match row.get(0) {
                None => return Err(DbError::new("Invalid column")),
                Some(acc_id) => acc_id,
            },
            None => return Err(DbError::new("Insert did not return new id.")),
        },
        Err(e) => return Err(DbError::new(&e.to_string())),
    };
    // If we got here, then acc_id is the id of the new account.
    // But we can't commit yet - we still need to add email.
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
        &[&acc_id, &email],
    ) {
        Ok(_) => {
            // Insert ok, let's commit...
            match transaction.commit() {
                Ok(_) => {
                    debug!("create_account_with_email: commit done");
                    Ok(acc_id)
                }
                Err(e) => Err(DbError::new(&e.to_string())),
            }
        }
        Err(e) => {
            // Insert failed...
            Err(DbError::new(&format!(
                "Failed to insert account_email: {}",
                e
            )))
        }
    }
}

pub fn get_user_account_emails(
    conn: &mut postgres::Client,
    acc_id: i64,
) -> Result<Box<Vec<String>>, DbError> {
    match conn.query(
        "select email from account_email where account = $1",
        &[&acc_id],
    ) {
        Ok(rows) => {
            let v = rows.iter().map(|r| r.get(0)).collect();
            let b = Box::new(v);
            Ok(b)
        }
        Err(e) => Err(DbError::new(&format!(
            "Error while getting account emails: {}",
            e
        ))),
    }
}

pub fn get_user_account_info(
    conn: &mut postgres::Client,
    acc_id: i64,
) -> Result<Option<AccountInfo>, DbError> {
    let emails = get_user_account_emails(conn, acc_id)?;
    match conn.query(
        "select
            to_char(
                created,
                'YYYY-MM-DD HH24:MI:SS.US'),
            to_char(
                modified,
                'YYYY-MM-DD HH24:MI:SS.US')
        from account
        where id = $1",
        &[&acc_id],
    ) {
        Ok(rows) => match rows.iter().next() {
            Some(row) => {
                let created = chrono::Utc
                    .datetime_from_str(row.get(0), "%Y-%m-%d %H:%M:%S%.f")
                    .unwrap();
                let modified = chrono::Utc
                    .datetime_from_str(row.get(1), "%Y-%m-%d %H:%M:%S%.f")
                    .unwrap();
                let acc_info = AccountInfo {
                    created_at: created,
                    modified_at: modified,
                    emails: emails,
                };
                Ok(Some(acc_info))
            }
            None => Ok(None),
        },
        Err(e) => Err(DbError::new(&format!(
            "Error getting user account info: {}",
            e
        ))),
    }
}

pub fn insert_login_token(
    conn: &mut postgres::Client,
    account_id: &i64,
    token: &str,
) -> Result<(), DbError> {
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
        )",
        &[&account_id, &token],
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(DbError::new(&e.to_string())),
    }
}

/// Get account id by login token.
/// Only active tokens are queried.
pub fn get_login_token_account(
    conn: &mut postgres::Client,
    token: &str,
) -> Result<Option<i64>, DbError> {
    match conn.query(
        "select account
        from login_token
        where token = $1 and used = false",
        &[&token],
    ) {
        Err(e) => Err(DbError::new(&e.to_string())),
        Ok(rows) => match rows.iter().next() {
            Some(row) => Ok(row.get(0)),
            None => Ok(None),
        },
    }
}

pub fn insert_entry(
    conn: &mut postgres::Client,
    account_id: &i64,
    bank_account: &i64,
    ts: &chrono::DateTime<chrono::Utc>,
    amount_str: &str,
) -> Result<(), DbError> {
    match conn.execute(
        "insert into entry (
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
            to_timestamp(
                $3,
                'YYYY-MM DD HH24:MI:SS.US'
            ),                 -- ts
            $4::text::numeric, -- amount
            false,
            current_timestamp,
            current_timestamp
        )",
        &[
            &account_id,
            &bank_account,
            &ts.format("%Y-%m-%d %H:%M:%S%.6f").to_string(),
            &amount_str,
        ],
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(DbError::new(&format!("Failed to insert entry: {}", e))),
    }
}

pub fn get_entry(
    conn: &mut postgres::Client,
    acc_id: i64,
    entry_id: i64,
) -> Result<Option<EntryInfo>, DbError> {
    let sql = "
        select
            entry.id,
            entry.amount::text,
            to_char(
                entry.ts,
                'YYYY-MM-DD HH24:MI:SS.US'),
            bank_account.name,
            bank_account.currency
        from
            entry
            join bank_account on (bank_account.id = entry.bank_account)
        where
            bank_account.account = $1
            and entry.id = $2
            and entry.deleted = false
            and bank_account.deleted = false
    ";
    let rows = conn.query(sql, &[&acc_id, &entry_id])?;
    match rows.iter().next() {
        Some(row) => {
            let id = row.get(0);
            let amount = row.get(1);
            let ts = chrono::Utc
                .datetime_from_str(row.get(2), "%Y-%m-%d %H:%M:%S%.f")
                .unwrap();
            let bank_account_name = row.get(3);
            let bank_account_currency = row.get(4);
            Ok(Some(EntryInfo {
                amount: amount,
                bank_account: bank_account_name,
                currency: bank_account_currency,
                id: id,
                ts: ts,
            }))
        }
        None => Ok(None),
    }
}

pub fn update_entry_amount(
    conn: &mut postgres::Client,
    account_id: i64,
    entry_id: i64,
    amount: String,
) -> Result<(), DbError> {
    let sql = "
        update entry
        set
            amount = $1::text::numeric,
            modified = current_timestamp
        where
            id = (
                select
                    entry.id
                from
                    entry
                    join bank_account on (bank_account.id = entry.bank_account)
                where
                    entry.id = $2
                    and bank_account.account = $3
                    and entry.deleted = false
                    and bank_account.deleted = false
            )
    ";
    conn.execute(sql, &[&amount, &entry_id, &account_id])?;
    Ok(())
}

/// Get CashLog entries for given account id.
/// Currently amount is String on Rust side - probably should be pair
/// of ints or something.
/// Returns only those entries that were not deleted (are not marked deleted).
pub fn get_entries(
    conn: &mut postgres::Client,
    account_id: i64,
) -> Result<Vec<EntryInfo>, DbError> {
    let sql = "select
            entry.id,
            bank_account.name,
            entry.amount::text,
            bank_account.currency,
            to_char(entry.ts, 'YYYY-MM-DD HH24:MI:SS.US')
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
        Ok(rows) => Ok(rows
            .iter()
            .map(|row| EntryInfo {
                id: row.get(0),
                bank_account: row.get(1),
                amount: row.get(2),
                currency: row.get(3),
                ts: chrono::Utc
                    .datetime_from_str(row.get(4), "%Y-%m-%d %H:%M:%S%.f")
                    .unwrap(),
            })
            .collect()),
        Err(e) => Err(DbError::new(&e.to_string())),
    }
}

pub fn get_entries_by_bank_account(
    conn: &mut postgres::Client,
    account_id: i64,
    bank_account_name: &str,
) -> Result<Vec<EntryInfo>, DbError> {
    let sql = "
        select
            entry.id,
            bank_account.name,
            entry.amount::text,
            bank_account.currency,
            to_char(
                entry.ts,
                'YYYY-MM-DD HH24:MI:SS.US'
            )
        from
            entry
            join bank_account on (bank_account.id = entry.bank_account)
        where
            bank_account.account = $1
            and bank_account.name = $2
            and entry.deleted = false
            and bank_account.deleted = false
        order by entry.ts
        limit 4096";
    match conn.query(sql, &[&account_id, &bank_account_name]) {
        Ok(rows) => Ok(rows
            .iter()
            .map(|row| EntryInfo {
                id: row.get(0),
                bank_account: row.get(1),
                amount: row.get(2),
                currency: row.get(3),
                ts: chrono::Utc
                    .datetime_from_str(row.get(4), "%Y-%m-%d %H:%M:%S%.f")
                    .unwrap(),
            })
            .collect()),
        Err(err) => Err(DbError::new(&err.to_string())),
    }
}

/// Delete entry.
/// The account_id is redundant, but we use it for security.
pub fn delete_entry(
    conn: &mut postgres::Client,
    account_id: i64,
    entry_id: i64,
) -> Result<(), DbError> {
    conn.execute(
        "update entry
                 set deleted = true, modified = current_timestamp
                 where id = $2
                 and bank_account in (select id from bank_account where account = $1)",
        &[&account_id, &entry_id],
    )?;
    Ok(())
}

pub fn get_bank_accounts(
    conn: &mut postgres::Client,
    account_id: i64,
) -> Result<Vec<BankAccount>, DbError> {
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
        Ok(rows) => Ok(rows
            .iter()
            .map(|row| BankAccount {
                id: row.get(0),
                name: row.get(1),
                currency: row.get(2),
            })
            .collect()),
        Err(err) => Err(DbError::from(err)),
    }
}

pub fn get_bank_account_infos(
    conn: &mut postgres::Client,
    account_id: i64,
) -> Result<Vec<BankAccountInfo>, DbError> {
    let sql = "
        select
            bank_account_and_last_entry_id.name,
            last_entry.amount::text,
            bank_account_and_last_entry_id.currency,
            to_char(
                last_entry.ts,
                'YYYY-MM-DD HH24:MI:SS.US'
            )
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
            ) as bank_account_and_last_entry_id
            left join entry as last_entry on (
                last_entry.id = bank_account_and_last_entry_id.last_entry_id
            )
        where
            bank_account_and_last_entry_id.account = $1
            and bank_account_and_last_entry_id.deleted = false
            and bank_account_and_last_entry_id.last_entry_id is not null
        order by
            bank_account_and_last_entry_id.name,
            bank_account_and_last_entry_id.currency
    ";
    match conn.query(sql, &[&account_id]) {
        Ok(rows) => Ok(rows
            .iter()
            .map(|row| BankAccountInfo {
                bank_account: row.get(0),
                amount: row.get(1),
                currency: row.get(2),
                ts: chrono::Utc
                    .datetime_from_str(row.get(3), "%Y-%m-%d %H:%M:%S%.f")
                    .unwrap(),
            })
            .collect()),
        Err(e) => Err(DbError::from(e)),
    }
}

pub fn insert_bank_account(
    conn: &mut postgres::Client,
    account_id: i64,
    name: &str,
    currency: &str,
) -> Result<(), DbError> {
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

pub fn get_currency_info(
    conn: &mut postgres::Client,
    account_id: i64,
) -> Result<Vec<CurrencyInfo>, DbError> {
    let sql = "
        select
            -- Finally, group those selected entries to produce summaries.
            currency,
            sum(amount)::text,
            to_char(
                max(ts),
                'YYYY-MM-DD HH24:MI:SS.US'
            )
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
            join entry as last_entry
                on (last_entry.id = bank_account_last_entry_id.entry_id)
        group by currency
        order by currency";
    let rows = conn.query(sql, &[&account_id])?;
    Ok(rows
        .iter()
        .map(|row| CurrencyInfo {
            currency: row.get(0),
            amount: row.get(1),
            ts: chrono::Utc
                .datetime_from_str(row.get(2), "%Y-%m-%d %H:%M:%S%.f")
                .unwrap(),
        })
        .collect())
}
