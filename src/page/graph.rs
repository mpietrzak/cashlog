
use iron;

use common;
use db;
use model::EntryInfo;
use params::{Params, Value};
use plugin::Pluggable; // get_ref
use tmpl;


pub fn handle_graph(request: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let bank_account = {
        let map = request.get_ref::<Params>().unwrap();
        match map.find(&["account"]) {
            Some(&Value::String(ref bank_account)) => bank_account.clone(),
            _ => return Ok(iron::Response::with(iron::status::NotFound))
        }
    };
    let mut conn = itry!(common::get_pooled_db_connection(request));
    let o_account_id = common::get_session_account_id(&mut conn, request);
    if let Some(account_id) = o_account_id {
        let entries: Vec<EntryInfo> = itry!(db::get_entries_by_bank_account(
            &mut conn,
            account_id,
            &bank_account));
        let resp_html = tmpl::graph::tmpl_graph(&entries);
        Ok(iron::Response::with((iron::status::Ok, resp_html)))
    } else {
        Ok(itry!(common::redirect(request, "new-session")))
    }
}
