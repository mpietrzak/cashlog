use actix_web;
use actix_web::HttpMessage;
use actix_web::Responder;

use crate::common;
use crate::db;
use crate::tmpl::edit::tmpl_edit;
use crate::tmpl::edit::FormData;

/// The query or post params of the edit page.
#[derive(Deserialize)]
pub struct EditParams {
    /// The id of the entry to edit.
    id: String,
}

/// Params of the page where user saves the edits.
#[derive(Deserialize)]
pub struct EditPostParams {
    id: String,
    amount: String,
}

pub async fn handle_edit(
    request: actix_web::HttpRequest,
    pool: actix_web::web::Data<common::DatabasePool>,
    params: actix_web::web::Query<EditParams>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let cookie = request.cookie("session").unwrap();
    let sess_key = cookie.value();
    let account_id = match db::get_sess_val(&mut conn, sess_key, "account") {
        Some(account_id) => account_id.parse().unwrap(),
        None => {
            return actix_web::HttpResponse::SeeOther()
                .header("Location", "new-session")
                .body("Redirecting to new session form");
        }
    };
    let entry = match db::get_entry(&mut conn, account_id, params.id.parse().unwrap()).unwrap() {
        Some(entry) => entry,
        None => {
            return actix_web::HttpResponse::InternalServerError().body("No such entry");
        }
    };
    let form_data = FormData {
        id: entry.id,
        amount: (String::from(entry.amount), None),
    };
    let resp_body = tmpl_edit(&form_data).into_string();
    actix_web::HttpResponse::Ok().body(resp_body)
}

pub async fn handle_post_edit(
    request: actix_web::HttpRequest,
    pool: actix_web::web::Data<common::DatabasePool>,
    params: actix_web::web::Form<EditPostParams>,
) -> impl actix_web::Responder {
    let mut conn = pool.get().unwrap();
    let cookie = request.cookie("session").unwrap();
    let sess_key = cookie.value();
    let account_id = db::get_sess_val(&mut conn, sess_key, "account")
        .unwrap()
        .parse()
        .unwrap();
    if db::get_entry(&mut conn, account_id, params.id.parse().unwrap())
        .unwrap()
        .is_none()
    {
        return actix_web::HttpResponse::NotFound().body("Not found");
    };
    db::update_entry_amount(
        &mut conn,
        account_id,
        params.id.parse().unwrap(),
        params.amount.clone(),
    )
    .unwrap();
    actix_web::HttpResponse::SeeOther()
        .header("Location", ".")
        .body("Redirecting...")
}
