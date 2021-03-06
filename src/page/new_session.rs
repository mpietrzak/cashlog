
//! Handle login.

use cookie::Cookie;
use iron;
use iron::headers::SetCookie;
use iron::mime::Mime;
use params::Params;
use plugin::Pluggable;
use router::Router;
use time::Duration;
use uuid;

use common;
use db;
use model;
use tmpl::new_session::tmpl_new_session;
use tmpl::new_session::tmpl_new_session_email_sent;
use tmpl::new_session::tmpl_new_session_result;
use util::get_str;

pub fn handle_new_session(_: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let resp_html = tmpl_new_session().into_string();
    let ct = "text/html".parse::<Mime>().unwrap();
    Ok(iron::response::Response::with(
        (iron::status::Ok, ct, resp_html),
    ))
}

/// Use clicked "yes" on login via email form.
/// We have to:
/// - check if email belongs to existing account,
///   - if not, create empty account with new id,
///   - if yes, then get account id we're trying to log in,
///   - the DB constraint prevent race here,
/// - generate login token,
/// - store it in login token table,
/// - send an email to given user
///   - if user is in fact able to read email, then they'll be able to
///     setup the session.
pub fn handle_post_new_session(r: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let o_email = handle_post_new_session_form(r);
    let mut conn = itry!(common::get_pooled_db_connection(r));
    match o_email {
        Some(email) => {
            let account_id: i64 = match db::get_account_id_by_email(&mut conn, &email) {
                Ok(oa) => match oa {
                    Some(a) => a,
                    None => itry!(db::create_account_with_email(&mut conn, &email)),
                },
                Err(e) => {
                    return Err(iron::IronError::new(
                        e,
                        (iron::status::InternalServerError, "Failed to query account"),
                    ))
                }
            };
            let token: String = uuid::Uuid::new_v4().to_string();
            let use_email: bool = {
                r.extensions
                    .get::<model::Config>()
                    .map_or(false, |c| c.use_email)
            };
            let base_url = itry!(common::get_base_url(r));
            itry!(db::insert_login_token(&mut conn, &account_id, &token));
            itry!(common::send_email_login_email(
                &base_url,
                &email,
                &token,
                use_email
            ));
            let resp_content_type = "text/html".parse::<Mime>().unwrap();
            let resp_html = tmpl_new_session_email_sent().into_string();
            Ok(iron::response::Response::with(
                (iron::status::Ok, resp_content_type, resp_html),
            ))
        }
        None => {
            let resp_html = tmpl_new_session().into_string();
            Ok(iron::Response::with((
                iron::status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                resp_html,
            )))
        }
    }
}

fn set_session_cookie(resp: &mut iron::Response, session_key: &str) -> Result<(), common::Error> {
    let session_cookie = Cookie::build("session", String::from(session_key))
        .path("/")
        .max_age(Duration::days(365))
        .finish();
    resp.headers
        .set(SetCookie(vec![session_cookie.to_string()]));
    Ok(())
}

pub fn get_request_token(r: &iron::Request) -> Option<String> {
    r.extensions
        .get::<Router>()
        .unwrap()
        .find("token")
        .map(|s| String::from(s))
}


/// User clicks on login link in email.
/// URL contains the token.
/// If token looks good, then we'll give user's browser the session cookie.
/// We also need to mark token as consumed.
pub fn handle_get_new_session_token(r: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let mut conn = itry!(common::get_pooled_db_connection(r));
    let mk = get_request_token(r);
    match mk {
        Some(login_token) => {
            debug!("Logging in with key {}.", login_token);
            match db::get_login_token_account(&mut conn, &login_token) {
                Err(e) => {
                    return Err(iron::IronError::new(
                        e,
                        (iron::status::InternalServerError, "Failed to check token"),
                    ))
                }
                Ok(oa) => {
                    match oa {
                        None => Ok(iron::Response::with((iron::status::Ok, "Invalid token"))),
                        Some(account_id) => {
                            /// Yeah, token is ok.
                            /// TODO: mark token as used.
                            let session_key: String = uuid::Uuid::new_v4().to_string();
                            itry!(db::set_session_value(
                                &mut conn,
                                &session_key,
                                "account",
                                &format!("{}", account_id)
                            ));
                            let ct = "text/html".parse::<Mime>().unwrap();
                            let mut resp = iron::Response::with((
                                iron::status::Ok,
                                ct,
                                tmpl_new_session_result(true).into_string(),
                            ));
                            itry!(set_session_cookie(&mut resp, &session_key));
                            Ok(resp)
                        }
                    }
                }
            }
        }
        None => {
            debug!("No token param found in URL.");
            let ct = "text/html".parse::<Mime>().unwrap();
            Ok(iron::Response::with((
                iron::status::Ok,
                ct,
                tmpl_new_session_result(false).into_string(),
            )))
        }
    }
}

/// Extract stuff from the posted form.
fn handle_post_new_session_form(r: &mut iron::Request) -> Option<String> {
    let params = r.get_ref::<Params>().unwrap();
    let r_email = get_str(params, "email");
    match r_email {
        Ok(email) => Some(email),
        Err(_) => None,
    }
}
