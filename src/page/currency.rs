//! Currency view.
use actix_web;
use actix_web::HttpMessage;

use crate::common;
use crate::db;
use crate::tmpl::currency::tmpl_currency;

pub async fn handle_currency(
    request: actix_web::HttpRequest,
    pool: actix_web::web::Data<common::DatabasePool>,
) -> impl actix_web::Responder {
    let mut conn = pool.get().unwrap();
    let sess_cookie = request.cookie("session").unwrap();
    let sess_key = sess_cookie.value();
    let acc_id = db::get_sess_val(&mut conn, sess_key, "account")
        .unwrap()
        .parse()
        .unwrap();
    let currency_info = db::get_currency_info(&mut conn, acc_id).unwrap();
    let content = tmpl_currency(currency_info).into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(content)
}
