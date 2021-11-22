use actix_web::web::Data;
use actix_web::HttpMessage;
use actix_web::Responder;

use crate::common;
use crate::db;
use crate::model;
use crate::tmpl;

#[derive(Deserialize)]
pub struct HandleExportFileParams {
    pub filename: String,
}

fn entries_to_csv(entries: Vec<model::EntryInfo>) -> String {
    let mut csv: String = String::new();
    csv.push_str("ts,account,amount,currency\n");
    for e in entries {
        let ts_str = e.ts.format("%Y-%m-%d %H:%M:%S%.f");
        csv.push_str(&format!(
            "{},{},{},{}\n",
            ts_str, e.bank_account, e.amount, e.currency
        ));
    }
    csv
}

/// Show export page.
pub async fn handle_export(
    req: actix_web::HttpRequest,
    config: Data<model::Config>,
    pool: actix_web::web::Data<common::DatabasePool>,
) -> impl Responder {
    let sess_cookie = req.cookie("session").expect("Request has no cookie");
    let sess_key = sess_cookie.value();
    let mut conn = pool.get().unwrap();
    if db::get_sess_val(&mut conn, &sess_key, "account").is_none() {
        return actix_web::HttpResponse::SeeOther()
            .header("Location", ".")
            .body("Redirecting to the form");
    }
    let r = tmpl::export::tmpl_export(&config.base_url).into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(r)
}

/// Generate export file.
pub async fn handle_export_file(
    req: actix_web::HttpRequest,
    pool: actix_web::web::Data<common::DatabasePool>,
    params: actix_web::web::Path<HandleExportFileParams>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let sess_cookie = req.cookie("session").unwrap();
    let sess_key = sess_cookie.value();
    let acc_id = match db::get_sess_val(&mut conn, sess_key, "account") {
        Some(acc_id) => acc_id.parse().unwrap(),
        None => {
            return actix_web::HttpResponse::SeeOther()
                .header("Location", "new-session")
                .body("New session")
        }
    };
    let entries = db::get_entries(&mut conn, acc_id).unwrap();
    let csv = entries_to_csv(entries);
    actix_web::HttpResponse::Ok()
        .content_type("text/csv")
        .header("Content-Disposition", params.filename.to_string())
        .body(csv)
}
