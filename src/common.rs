//! Shared code in CashLog.
//! This code can use other parts of CashLog.

use std;

use r2d2;
use r2d2_postgres;
use url;

pub type DatabasePool =
    r2d2::Pool<r2d2_postgres::PostgresConnectionManager<r2d2_postgres::postgres::NoTls>>;

#[derive(Debug)]
pub struct Error {
    desc: String,
}

impl Error {
    pub fn new(desc: &str) -> Error {
        Error {
            desc: String::from(desc),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "cashlog::Error: {}", self.desc)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.desc
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::new(&format!("Error: url::ParseError: {}", err))
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::new(&format!("Error: {}", err))
    }
}

/// Create database pool, die if can't create.
pub fn create_database_pool(
    host: &str,
    port: u16,
    database_name: &str,
    username: &str,
    password: &str,
) -> DatabasePool {
    let config: postgres::config::Config = {
        let mut config = r2d2_postgres::postgres::Config::new();
        config.host(host);
        config.port(port);
        config.dbname(database_name);
        config.user(username);
        config.password(password);
        config
    };
    let manager =
        r2d2_postgres::PostgresConnectionManager::new(config, r2d2_postgres::postgres::NoTls);
    let pool = r2d2::Pool::builder()
        .max_size(100)
        .min_idle(Some(0))
        .max_lifetime(Some(std::time::Duration::from_secs(60)))
        .idle_timeout(Some(std::time::Duration::from_secs(10)))
        .build(manager)
        .expect("Failed to create R2D2 Pool");
    pool
}

/// Send the login email.
pub fn send_email_login_email(
    base_url: &str,
    email: &str,
    key: &str,
    use_email: bool,
) -> Result<(), Error> {
    use lettre::Transport;
    let url = format!("{}/new-session/{}", base_url, key);
    let body = format!("Click this link to login to CashLog: {}", url);
    if use_email {
        let m = lettre_email::EmailBuilder::new()
            .to(email)
            .from("cashlog@hell.cx")
            .subject("CashLog Email Login Link")
            .text(&body)
            .build()
            .unwrap();
        let smtp_client =
            lettre::smtp::SmtpClient::new_unencrypted_localhost().expect("Error creating client");
        let mut mailer = lettre::smtp::SmtpTransport::new(smtp_client);
        debug!("Sending email:\n{}", body);
        match mailer.send(m.into()) {
            Ok(_) => {
                debug!("Sent mail to {}.", email);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to send email to {}: {}.", email, e);
                Err(Error::new(&e.to_string()))
            }
        }
    } else {
        info!("Not sending email:\n{}", body);
        Ok(())
    }
}
