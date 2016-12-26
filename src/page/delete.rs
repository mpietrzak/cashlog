
use iron;
use plugin::Pluggable;
use params;

use common;
use db;

pub fn handle_delete(request: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let mut conn = db::connect();
    let redirect_response = itry!(common::redirect(request, "."));
    let acc_id = match common::get_session_account_id(&mut conn, request) {
        Some(acc_id) => acc_id,
        None => {
            return Ok(redirect_response)
        }
    };
    let entry_id = {
        let params = {
            let params = itry!(request.get_ref::<params::Params>());
            params
        };
        match params.find(&["id"]) {
            Some(&params::Value::String(ref id_str)) => itry!(id_str.parse()),
            _ => return Ok(redirect_response)
        }
    };
    itry!(db::delete_entry(&mut conn, acc_id, entry_id));
    Ok(redirect_response)
}
