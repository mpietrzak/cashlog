use actix_web;
use actix_web::HttpMessage;

use crate::common;
use crate::db;
use crate::tmpl;

pub async fn handle_get_logout() -> impl actix_web::Responder {
    let body = tmpl::logout::tmpl_logout().into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(body)
}

pub async fn handle_post_logout(
    pool: actix_web::web::Data<common::DatabasePool>,
    request: actix_web::HttpRequest,
) -> impl actix_web::Responder {
    let sess_cookie = request.cookie("session").unwrap();
    let sess_key = sess_cookie.value();
    let mut conn = pool.get().unwrap();
    db::delete_session(&mut conn, &sess_key).unwrap();
    actix_web::HttpResponse::SeeOther()
        .header("Location", "/")
        .body("Redirecting...")
}
