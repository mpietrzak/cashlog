use actix_web::HttpMessage;

use crate::common;
use crate::db;
use crate::model::EntryInfo;
use crate::tmpl;

pub async fn handle_main(
    req: actix_web::HttpRequest,
    pool: actix_web::web::Data<common::DatabasePool>,
) -> impl actix_web::Responder {
    let mut conn = pool.get().unwrap();
    let sess_cookie = match req.cookie("session") {
        Some(c) => c,
        None => return actix_web::HttpResponse::SeeOther().header("Location", "new-session").body("Redirecting..."),
    };
    let sess_key = sess_cookie.value();
    let o_account_id = db::get_sess_val(&mut conn, sess_key, "account");
    if let Some(account_id) = o_account_id {
        let entries: Vec<EntryInfo> =
            db::get_entries(&mut conn, account_id.parse().unwrap()).unwrap();
        let resp_html = tmpl::main::tmpl_main("Main", &entries).into_string();
        let ct = "text/html";
        actix_web::HttpResponse::Ok()
            .content_type(ct)
            .body(resp_html)
    } else {
        actix_web::HttpResponse::SeeOther()
            .header("Location", "new-session")
            .body("Redirecting to new session form")
    }
}
