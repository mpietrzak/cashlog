
//! Handle login.

use iron::headers::CookiePair;
use iron::headers::CookieJar;
use iron::headers::SetCookie;
use iron::mime::Mime;
use iron::prelude::*;
use iron;
use params::Params;
use router::Router;
use uuid;

use common;
use db;
use model;
use tmpl::new_session::tmpl_new_session;
use tmpl::new_session::tmpl_new_session_email_sent;
use tmpl::new_session::tmpl_new_session_result;
use util::get_str;

pub fn handle_new_session(_: &mut Request) -> IronResult<Response> {
    let resp_html = tmpl_new_session();
    let resp_content_type = "text/html".parse::<Mime>().unwrap();
    Ok(Response::with((resp_content_type, iron::status::Ok, resp_html)))
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
pub fn handle_post_new_session(r: &mut Request) -> IronResult<Response> {
    let o_email = handle_post_new_session_form(r);
    let mut conn = db::connect();
    match o_email {
        Some(email) => {
            let account_id: i64 = match db::get_account_id_by_email(&mut conn, &email) {
                Ok(oa) => {
                    match oa {
                        Some(a) => a,
                        None => {
                            match db::create_account_with_email(&mut conn, &email) {
                                Ok(id) => id,
                                Err(e) => {
                                    /// oops
                                    return Err(iron::IronError::new(
                                        e,
                                        (iron::status::InternalServerError, "Failed to create account")
                                    ));
                                }
                            }
                        }
                    }
                }
                Err(e) => return Err(
                    iron::IronError::new(
                        e,
                        (iron::status::InternalServerError, "Failed to query account")
                    ))
            };
            let token: String = uuid::Uuid::new_v4().to_string();
            let use_email: bool = {
                r.extensions.get::<model::Config>().map_or(false, |c| c.use_email)
            };
            let base_url = common::get_base_url(r);
            itry!(db::insert_login_token(&mut conn, &account_id, &token));
            itry!(common::send_email_login_email(
                base_url.as_ref().map(String::as_ref),
                &email,
                &token,
                use_email));
            // let resp_content_type = "text/html".parse::<Mime>().unwrap();
            let resp_html = tmpl_new_session_email_sent();
            Ok(Response::with((iron::status::Ok, resp_html)))
        }
        None => {
         let resp_html = tmpl_new_session();
         Ok(Response::with((iron::status::Ok, resp_html)))
        }
    }
}

fn set_session_cookie(resp: &mut Response, session_key: &str) -> Result<(), common::Error> {
    let mut session_cookie = CookiePair::new(
        "session".to_string(),
        session_key.to_string());
    session_cookie.max_age = Some(20 * 365 * 24 * 3600);
    session_cookie.path = Some("/".to_string());
    let jar = CookieJar::new(common::COOKIE_KEY);
    let signed_jar = jar.signed();
    signed_jar.add(session_cookie);
    // resp.headers.set(SetCookie(vec![session_cookie]));
    let set_cookie_header = jar.delta();
    resp.headers.set(SetCookie(set_cookie_header));
    Ok(())
}

pub fn get_request_token(r: &Request) -> Option<String> {
    r.extensions.get::<Router>().unwrap().find("token").map(|s| String::from(s))
}


/// User clicks on login link in email.
/// URL contains the token.
/// If token looks good, then we'll give user's browser the session cookie.
/// We also need to mark token as consumed.
pub fn handle_get_new_session_token(r: &mut Request) -> IronResult<Response> {
    let mut conn = db::connect();
    let mk = get_request_token(r);
    match mk {
        Some(login_token) => {
            debug!("Logging in with key {}.", login_token);
            match db::get_login_token_account(&mut conn, &login_token) {
                Err(e) => return Err(iron::IronError::new(e, (iron::status::InternalServerError, "Failed to check token"))),
                Ok(oa) => match oa {
                    None => Ok(iron::Response::with((iron::status::Ok, "Invalid token"))),
                    Some(account_id) => {
                        /// Yeah, token is ok.
                        /// TODO: mark token as used.
                        let session_key: String = uuid::Uuid::new_v4().to_string();
                        itry!(db::set_session_value(
                            &mut conn,
                            &session_key,
                            "account",
                            &format!("{}", account_id)));
                        let mut resp = iron::Response::with((iron::status::Ok, tmpl_new_session_result(true)));
                        itry!(set_session_cookie(&mut resp, &session_key));
                        Ok(resp)
                    }
                }
            }
        }
        None => {
            debug!("No token param found in URL.");
            Ok(iron::Response::with((iron::status::Ok, tmpl_new_session_result(false))))
        }
    }
}

/// Extract stuff from the posted form.
fn handle_post_new_session_form(r: &mut Request) -> Option<String> {
    let params = r.get_ref::<Params>().unwrap();
    let r_email = get_str(params, "email");
    match r_email {
        Ok(email) => Some(email),
        Err(_) => None
    }
}
