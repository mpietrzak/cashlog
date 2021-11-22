use crate::tmpl;

pub async fn handle_about() -> impl actix_web::Responder {
    let content = tmpl::about::tmpl_about().into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(content)
}
