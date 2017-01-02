
use iron;

use db;
use common;

pub fn handle_get_logout(req: &mut iron::Request) -> iron::IronResult<iron::Response> {
    if let Ok(ms) = common::get_session_id(req) {
        if let Some(s) = ms {
            let mut conn = itry!(common::get_pooled_db_connection(req));
            itry!(db::delete_session(&mut conn, &s));
        }
    }
    Ok(itry!(common::redirect(req, "/")))
}
