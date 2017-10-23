
//! The profile page.

use iron;
use mime::Mime;

use common;
use db;
use tmpl;

pub fn handle_profile(req: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let mut conn = itry!(common::get_pooled_db_connection(req));
    match common::get_session_account_id(&mut conn, req) {
        Some(acc_id) => match itry!(db::get_user_account_info(&mut conn, acc_id)) {
            None => Ok(iron::Response::with(iron::status::NotFound)),
            Some(acc) => {
                let content = tmpl::profile::tmpl_profile(&acc).into_string();
                let ct = "text/html".parse::<Mime>().unwrap();
                Ok(iron::Response::with((iron::status::Ok, ct, content)))
            }
        },
        None => Ok(itry!(common::redirect(req, "/new-session"))),
    }
}
