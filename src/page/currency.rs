
//! Currency view.

use iron;
use iron::IronResult;
use iron::Response;
use mime::Mime;

use common;
use db;
use tmpl::currency::tmpl_currency;

pub fn handle_currency(request: &mut iron::Request) -> IronResult<Response> {
    let pool = request
        .extensions
        .get::<common::DatabasePool>()
        .unwrap()
        .clone();
    let mut conn = itry!(pool.get());
    let account_id = match common::get_session_account_id(&mut conn, request) {
        Some(acc_id) => acc_id,
        None => return Ok(itry!(common::redirect(request, "."))),
    };
    let currency_info = itry!(db::get_currency_info(&mut conn, account_id));
    let content = tmpl_currency(currency_info).into_string();
    let ct = "text/html".parse::<Mime>().unwrap();
    Ok(Response::with((iron::status::Ok, ct, content)))
}
