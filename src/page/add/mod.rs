use actix_web::HttpMessage;

use crate::common;
use crate::db;
use crate::tmpl::add::tmpl_add;
use crate::util::parse_ts;

/// POST params to add an entry.
#[derive(Deserialize)]
pub struct AddPostParams {
    pub ts: String,
    pub amount: String,
    pub bank_account: i64,
}

pub async fn handle_add(
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
    let bank_accounts = db::get_bank_accounts(&mut conn, acc_id).unwrap();
    let resp_html = tmpl_add("Add", &bank_accounts).into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(resp_html)
}

pub async fn handle_post_add(
    request: actix_web::HttpRequest,
    pool: actix_web::web::Data<common::DatabasePool>,
    params: actix_web::web::Form<AddPostParams>,
) -> impl actix_web::Responder {
    let mut conn = pool.get().unwrap();
    let sess_cookie = request.cookie("session").unwrap();
    let sess_key = sess_cookie.value();
    let acc_id = db::get_sess_val(&mut conn, sess_key, "account")
        .unwrap()
        .parse()
        .unwrap();
    db::insert_entry(
        &mut conn,
        &acc_id,
        &params.bank_account,
        &parse_ts(&params.ts).unwrap(),
        &params.amount,
    )
    .unwrap();
    actix_web::HttpResponse::SeeOther()
        .header("Location", ".")
        .body("Redirecting...")
}
