
use iron;
use mime::Mime;
use params;
use plugin::Pluggable;

use common;
use db;
use tmpl;

pub fn handle_get_logout(_: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let body = tmpl::logout::tmpl_logout().into_string();
    let ct = "text/html".parse::<Mime>().unwrap();
    Ok(iron::Response::with((iron::status::Ok, ct, body)))
}

fn get_confirm(request: &mut iron::Request) -> Option<String> {
    match request.get_ref::<params::Params>() {
        Ok(map) => match map.get("confirm") {
            Some(&params::Value::String(ref s)) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}

pub fn handle_post_logout(request: &mut iron::Request) -> iron::IronResult<iron::Response> {
    if let Some(session_id) = itry!(common::get_session_id(request)) {
        let confirm = get_confirm(request);
        match confirm {
            Some(ref confirm) if confirm == "yes" => {
                let mut conn = itry!(common::get_pooled_db_connection(request));
                itry!(db::delete_session(&mut conn, &session_id));
            }
            _ => (),
        }
    }
    Ok(itry!(common::redirect(request, "/")))
}
