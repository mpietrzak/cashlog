
use iron;
use mime::Mime;

use common;
use db;
use model;
use router;
use time;
use tmpl;
use mime;

fn entries_to_csv(entries: Vec<model::Entry>) -> String {
    let mut csv: String = String::new();
    csv.push_str("ts,account,amount,currency\n");
    for e in entries {
        let ts_str = time::strftime("%Y-%m-%d %H:%M:%S", &time::at_utc(e.ts)).unwrap();
        csv.push_str(&format!("{},{},{},{}\n",
            ts_str,
            e.bank_account,
            e.amount,
            e.currency));
    }
    csv
}

/// Extract the filename from request params.
/// TODO: make sure filename contains only allowed chars, otherwise we risk
/// XSS vuln.
fn get_export_filename_request_param(request: &iron::Request) -> Option<String> {
    request.extensions.get::<router::Router>().unwrap().find("filename").map(|s| String::from(s))
}

/// Show export page.
pub fn handle_export(request: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let base_url = itry!(common::get_base_url(&request));
    let r = tmpl::export::tmpl_export(&base_url).into_string();
    let ct = "text/html".parse::<Mime>().unwrap();
    Ok(iron::Response::with((iron::status::Ok, ct, r)))
}

/// Generate export file.
pub fn handle_export_file(request: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let filename = get_export_filename_request_param(request).unwrap_or(String::from("export.csv"));
    let pool = request.extensions.get::<common::DatabasePool>().unwrap().clone();
    let mut conn = pool.get().unwrap();
    let acc_id = match common::get_session_account_id(&mut conn, request) {
        Some(acc_id) => acc_id,
        None => return Ok(itry!(common::redirect(request, ".")))
    };
    let entries = itry!(db::get_entries(&mut conn, acc_id));
    let csv = entries_to_csv(entries);
    let csv_content_type: mime::Mime = "text/csv".parse().unwrap();
    let csv_content_disposition_header = iron::headers::ContentDisposition{
        disposition: iron::headers::DispositionType::Attachment,
        parameters: vec![iron::headers::DispositionParam::Filename(
            iron::headers::Charset::Ext(String::from("UTF-8")),
            None,
            filename.as_bytes().into() // the actual bytes of the filename
        )]
    };
    Ok(iron::Response::with((
        iron::status::Ok,
        csv_content_type,
        iron::modifiers::Header(csv_content_disposition_header),
        csv)))
}
