
use iron;
use iron::IronResult;
use iron::Request;
use iron::Response;
use mime::Mime;

use common;
use db;
use model::EntryInfo;
use tmpl;


pub fn handle_main(req: &mut Request) -> IronResult<Response> {
    let mut conn = itry!(common::get_pooled_db_connection(req));
    let o_account_id = common::get_session_account_id(&mut conn, req);
    if let Some(account_id) = o_account_id {
        let entries: Vec<EntryInfo> = itry!(db::get_entries(&mut conn, account_id));
        let resp_html = tmpl::main::tmpl_main("Main", &entries).into_string();
        let ct = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((iron::status::Ok, ct, resp_html)))
    } else {
        Ok(itry!(common::redirect(req, "new-session")))
    }
}
