
use iron;
use mime::Mime;

use common;
use db;
use tmpl;

pub fn handle_bank_accounts(request: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let mut conn = itry!(common::get_pooled_db_connection(request));
    let acc_id = match common::get_session_account_id(&mut conn, request) {
        Some(acc_id) => acc_id,
        None => return Ok(itry!(common::redirect(request, "."))),
    };
    let bank_accounts = itry!(db::get_bank_account_infos(&mut conn, acc_id));
    let content = tmpl::bank_accounts::tmpl_bank_accounts(&bank_accounts).into_string();
    let ct = "text/html".parse::<Mime>().unwrap();
    Ok(iron::Response::with((iron::status::Ok, ct, content)))
}
