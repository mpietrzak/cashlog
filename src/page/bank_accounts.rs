use actix_web::HttpMessage;

use crate::common;
use crate::db;
use crate::tmpl;

pub async fn handle_bank_accounts(
    pool: actix_web::web::Data<common::DatabasePool>,
    request: actix_web::HttpRequest,
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
    let bank_accounts = db::get_bank_account_infos(&mut conn, acc_id).unwrap();
    let content = tmpl::bank_accounts::tmpl_bank_accounts(&bank_accounts).into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(content)
}
