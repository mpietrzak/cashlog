use actix_web::HttpMessage;

use crate::common;
use crate::db;
use crate::model::EntryInfo;
use crate::tmpl;

#[derive(Deserialize)]
pub struct GraphParams {
    pub account: String,
}

pub async fn handle_graph(
    pool: actix_web::web::Data<common::DatabasePool>,
    request: actix_web::HttpRequest,
    params: actix_web::web::Query<GraphParams>,
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
    let account_id = db::get_sess_val(&mut conn, sess_key, "account")
        .unwrap()
        .parse()
        .unwrap();
    let entries: Vec<EntryInfo> =
        db::get_entries_by_bank_account(&mut conn, account_id, &params.account).unwrap();
    let resp_html = tmpl::graph::tmpl_graph(&entries).into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(resp_html)
}
