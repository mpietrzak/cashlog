//! The profile page.
use actix_web::HttpMessage;

use crate::common;
use crate::db;
use crate::tmpl;

pub async fn handle_profile(
    request: actix_web::HttpRequest,
    pool: actix_web::web::Data<common::DatabasePool>,
) -> impl actix_web::Responder {
    let mut conn = pool.get().unwrap();
    let cookie = match request.cookie("session") {
        Some(c) => c,
        None => {
            return actix_web::HttpResponse::SeeOther()
                .header("Location", "new-session")
                .body("Redirecting...")
        }
    };
    let sess_key = cookie.value();
    let acc_id = db::get_sess_val(&mut conn, sess_key, "account")
        .unwrap()
        .parse()
        .unwrap();
    let acc_info = db::get_user_account_info(&mut conn, acc_id)
        .unwrap()
        .unwrap();
    let content = tmpl::profile::tmpl_profile(&acc_info).into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(content)
}
