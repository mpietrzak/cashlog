use actix_web;
use actix_web::HttpMessage;

use crate::common;
use crate::db;

#[derive(Deserialize)]
pub struct DeletePostParams {
    pub id: i64,
}

pub async fn handle_delete(
    pool: actix_web::web::Data<common::DatabasePool>,
    request: actix_web::HttpRequest,
    params: actix_web::web::Query<DeletePostParams>,
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
    db::delete_entry(&mut conn, acc_id, params.id).unwrap();
    actix_web::HttpResponse::SeeOther()
        .header("Location", ".")
        .body("Redirecting...")
}
