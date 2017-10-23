
use iron;
use iron::IronResult;
use iron::Request;
use iron::Response;
use params::Params;
use params::Value;
use plugin::Pluggable;

use common;
use db;
use tmpl::edit::FormData;
use tmpl::edit::tmpl_edit;

pub fn handle_edit(request: &mut Request) -> IronResult<Response> {
    let mut conn = itry!(common::get_pooled_db_connection(request));
    let account_id = match common::get_session_account_id(&mut conn, request) {
        Some(account_id) => account_id,
        None => return Ok(itry!(common::redirect(request, "new-session"))),
    };
    let entry_id = {
        let map = request.get_ref::<Params>().unwrap();
        match map.find(&["id"]) {
            Some(&Value::String(ref v)) => v.parse().unwrap(),
            _ => return Ok(iron::Response::with(iron::status::NotFound)),
        }
    };
    let entry = match itry!(db::get_entry(&mut conn, account_id, entry_id)) {
        Some(entry) => entry,
        None => return Ok(Response::with("not found")),
    };
    let form_data = FormData {
        id: entry.id,
        amount: (String::from(entry.amount), None),
    };
    let resp = tmpl_edit(&form_data);
    Ok(Response::with(resp))
}

pub fn handle_post_edit(request: &mut Request) -> IronResult<Response> {
    let mut conn = itry!(common::get_pooled_db_connection(request));
    let account_id = match common::get_session_account_id(&mut conn, request) {
        Some(account_id) => account_id,
        None => return Ok(itry!(common::redirect(request, "new-session"))),
    };
    let entry_id = {
        let map = request.get_ref::<Params>().unwrap();
        match map.find(&["id"]) {
            Some(&Value::String(ref v)) => v.parse().unwrap(),
            _ => return Ok(iron::Response::with(iron::status::NotFound)),
        }
    };
    if itry!(db::get_entry(&mut conn, account_id, entry_id)).is_none() {
        return Ok(Response::with("not found"));
    };
    let amount_str = {
        match request.get_ref::<Params>().unwrap().find(&["amount"]) {
            Some(&Value::String(ref v)) => v.clone(),
            _ => return Ok(iron::Response::with(iron::status::NotFound)),
        }
    };
    match amount_str.parse::<f64>() {
        Ok(_) => {
            itry!(db::update_entry_amount(
                &mut conn,
                account_id,
                entry_id,
                amount_str
            ));
            Ok(itry!(common::redirect(request, "")))
        }
        Err(parse_error) => {
            let form_data = FormData {
                id: entry_id,
                amount: (amount_str, Some(parse_error.to_string())),
            };
            let resp = tmpl_edit(&form_data);
            Ok(Response::with(resp))
        }
    }
}
