//! Shared code in CashLog.
//! This code can use other parts of CashLog.

use std;

use iron;
use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::smtp::SmtpTransportBuilder;
use postgres;
use url;

use db;
use model;

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

/// Extract session id from request cookies.
pub fn get_session_id(request: &mut iron::Request) -> Result<Option<String>, Error> {
    match request.headers.get::<iron::headers::Cookie>() {
        Some(cookie) => {
            let jar = cookie.to_cookie_jar(COOKIE_KEY);
            let signed_jar = jar.signed();
            match signed_jar.find("session") {
                Some(c) => Ok(Some(c.value)),
                None => Ok(None)
            }
        }
        None => {
            Ok(None)
        }
    }
}

pub fn get_session_account_id(
        conn: &mut postgres::Connection,
        request: &mut iron::Request) -> Option<i64> {
    match get_session_id(request) {
        Ok(ms) => match ms {
            Some(s) => {
                let o_account_str = db::get_session_value(conn, &s, "account");
                match o_account_str {
                    Some(account_str) => {
                        match account_str.parse() {
                            Ok(account_id) => {
                                Some(account_id)
                            }
                            Err(e) => {
                                // This is a signed cookie, so it's
                                // a little bit strange if we can't parse.
                                warn!("Failed to parse string account id (\"{}\") into integer: {}",
                                    account_str,
                                    e);
                                None
                            }
                        }
                    }
                    None => {
                        None
                    }
                }
            }
            None => None
        },
        Err(e) => {
            warn!("Error while trying to get session id: {}", e);
            None
        }
    }
}

/// Get base URL from config or from request.
pub fn get_base_url(request: &iron::Request) -> Result<String, Error> {
    if let Some(conf) = request.extensions.get::<model::Config>() {
        if let Some(ref url) = conf.base_url {
            return Ok(url.clone());
        }
    }
    let url = &request.url;
    let scheme = url.scheme();
    let host = url.host();
    let port = url.port();
    Ok(format!("{}://{}:{}", scheme, host, port))
}

/// Create and return redirect response for given relative path.
/// Relative path is relative to site's base URL, or to scheme://host:port
/// of request connection.
pub fn redirect(request: &iron::Request, path: &str) -> Result<iron::Response, Error> {
    let base_url = get_base_url(request)?;
    let to_iron_url = iron::Url::parse(&format!("{}/{}", base_url, path))?;
    Ok(iron::Response::with((iron::status::Found, iron::modifiers::Redirect(to_iron_url))))
}

/// Send the login email.
pub fn send_email_login_email(
        base_url: &str,
        email: &str,
        key: &str,
        use_email: bool) -> Result<(), Error> {
    let url = format!("{}/new-session/{}", base_url, key);
    let body = format!("Click this link to login to CashLog: {}", url);
    if use_email {
        let m = EmailBuilder::new()
            .to(email)
            .from("cashlog@hell.cx")
            .subject("CashLog Email Login Link")
            .text(&body)
            .build()
            .unwrap();
        let mut mailer = SmtpTransportBuilder::localhost().unwrap().build();
        debug!("Sending email:\n{}", body);
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
