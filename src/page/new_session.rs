use uuid;

use crate::db;
use crate::tmpl::new_session::tmpl_new_session;
use crate::tmpl::new_session::tmpl_new_session_result;
use crate::common;
use crate::tmpl::new_session::tmpl_new_session_email_sent;

#[derive(Deserialize)]
pub struct GetNewSessionWithTokenParams {
    pub token: String,
}

/// The params of the "new session" page when invoked via POST.
#[derive(Deserialize)]
pub struct PostNewSessionParams {
    pub email: String,
}

/// The entry page for the new session flow, shows the basic form.
pub async fn handle_new_session() -> impl actix_web::Responder {
    let resp_html = tmpl_new_session().into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(resp_html)
}

/// Use clicked "yes" on login via email form.
/// We have to:
/// - check if email belongs to existing account,
///   - if not, create empty account with new id,
///   - if yes, then get account id we're trying to log in,
///   - the DB constraint prevent race here,
/// - generate the login token,
/// - store it in login token table,
/// - send an email to given user
///   - if user is in fact able to read email, then they'll be able to
///     setup the session.
pub async fn handle_post_new_session(
    config: actix_web::web::Data<crate::model::Config>,
    pool: actix_web::web::Data<common::DatabasePool>,
    params: actix_web::web::Form<PostNewSessionParams>) -> impl actix_web::Responder {
    let mut conn = pool.get().expect("Error getting db conn from pool");
    let acc_id: i64 = match db::get_acc_id_by_email(&mut conn, &params.email) {
        Ok(oa) => match oa {
            Some(a) => a,
            None => db::create_acc_with_email(&mut conn, &params.email).expect("Error creating an account"),
        },
        Err(_) => {
            return actix_web::HttpResponse::InternalServerError().body(
                "Failed to query account")
        }
    };
    let token: String = uuid::Uuid::new_v4().to_string();
    let use_email= config.use_email;
    db::insert_login_token(&mut conn, &acc_id, &token).expect("Error inserting login token");
    common::send_email_login_email(
        &config.base_url, &params.email, &token, use_email
    ).unwrap();
    let resp_html = tmpl_new_session_email_sent().into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body( resp_html)
}

/// User clicks on the login link in the new-session email.
/// URL contains the token.
/// If the token looks good, then we'll give user's browser the session cookie.
/// We also need to mark token as consumed.
/// This is a GET link, which kind of breaks the HTTP proto, maybe we should present a web page
/// where the user has a chance to consume the token by clicking a button?
pub async fn handle_get_new_session_with_token(
    pool: actix_web::web::Data<common::DatabasePool>,
    params: actix_web::web::Path<GetNewSessionWithTokenParams>) -> impl actix_web::Responder {
    use std::convert::TryInto;
    let mut conn = pool.get().expect("Error getting database conn from pool");
    debug!("Logging in with key {}.", &params.token);
    match db::get_login_token_account(&mut conn, &params.token) {
        Err(_) => {
            return actix_web::HttpResponse::InternalServerError().body("Failed to check token");
        }
        Ok(oa) => {
            match oa {
                None => actix_web::HttpResponse::BadRequest().body("Invalid token"),
                Some(acc_id) => {
                    // Yeah, token is ok.
                    // TODO: mark token as used.
                    let session_key: String = uuid::Uuid::new_v4().to_string();
                    db::set_session_value(
                        &mut conn,
                        &session_key,
                        "account",
                        &format!("{}", acc_id)
                    ).unwrap();
                    let cookie = actix_web::http::Cookie::build("session", session_key)
                        .path("/")
                        .secure(true)
                        .max_age(std::time::Duration::from_secs(60 * 60 * 24 * 365 * 2).try_into().expect("Error converting durations"))
                        .finish();
                    let resp = actix_web::HttpResponse::Ok()
                        .content_type("text/html")
                        .cookie(cookie)
                        .body(tmpl_new_session_result(true).into_string());
                    resp
                }
            }
        }
    }
}

