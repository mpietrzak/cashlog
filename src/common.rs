//! Shared code in CashLog.
//! This code can use other parts of CashLog.

use std;

use iron;
use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::smtp::SmtpTransport;
use lettre::transport::smtp::SmtpTransportBuilder;
use lettre::transport::stub::StubEmailTransport;
use postgres;

use db;

pub const COOKIE_KEY: &'static [u8] = b"2ac7b2d5-b4c0-4e0a-a945-b9b8dbf4fbcb";

#[derive(Debug)]
pub struct Error {
    desc: String
}

impl Error {
    pub fn new(desc: &str) -> Error {
        Error {
            desc: String::from(desc)
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

pub fn get_session_account_id(
        conn: &mut postgres::Connection,
        request: &mut iron::Request) -> Option<i64> {
    let o_cookie = request.headers.get::<iron::headers::Cookie>();
    let o_account_id = match o_cookie {
        Some(cookie) => {
            debug!("Request cookie header:\n{:?}", cookie);
            let jar = cookie.to_cookie_jar(COOKIE_KEY);
            let signed_jar = jar.signed();
            let o_session_cookie = signed_jar.find("session");
            match o_session_cookie {
                Some(session_cookie) => {
                    debug!("Session cookie: {:?}", session_cookie);
                    let session_key: String = session_cookie.value;
                    debug!("Session cookie value: {}", session_key);
                    let o_account_str = db::get_session_value(conn, &session_key, "account");
                    match o_account_str {
                        Some(account_str) => {
                            match account_str.parse() {
                                Ok(account_id) => {
                                    debug!("User is logged in as: {}", account_id);
                                    Some(account_id)
                                }
                                Err(e) => {
                                    warn!("Failed to parse string account id (\"{}\") into integer: {}",
                                        account_str,
                                        e);
                                    None
                                }
                            }
                        }
                        None => {
                            debug!("User is not logged in (session does not contain account variable).");
                            None
                        }
                    }
                }
                None => {
                    debug!("No session cookie in cookie header.");
                    None
                }
            }
        }
        None => {
            debug!("No cookie header.");
            None
        }
    };
    o_account_id
}

pub fn send_email_login_email(email: &str, key: &str, use_email: bool) -> Result<(), Error> {
    let body = format!("Click this link to login to CashLog: http://localhost:14080/new-session/{}", key);
    debug!("Sending email:\n{}", body);
    if use_email {
        let m = EmailBuilder::new()
            .to(email)
            .from("cashlog@hell.cx")
            .subject("CashLog Email Login Link")
            .text(&body)
            .build()
            .unwrap();
        let mut mailer = SmtpTransportBuilder::localhost().unwrap().build();
        match mailer.send(m) {
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
