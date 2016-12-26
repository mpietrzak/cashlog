
use iron;

use common;
use db;
use tmpl;

pub fn handle_bank_accounts(request: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let mut conn = db::connect();
    let acc_id = match common::get_session_account_id(&mut conn, request) {
        Some(acc_id) => acc_id,
        None => return Ok(itry!(common::redirect(request, ".")))
    };
    let bank_accounts = itry!(db::get_bank_accounts(&mut conn, acc_id));
    let content = tmpl::bank_accounts::tmpl_bank_accounts(&bank_accounts);
    Ok(iron::Response::with((iron::status::Ok, content)))
}
