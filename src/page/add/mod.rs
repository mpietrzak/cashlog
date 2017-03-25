
use std::collections::BTreeMap;
use std::collections::HashMap;

use iron::mime::Mime;
use iron;
use iron::Request;
use iron::IronResult;
use iron::Response;
use params::Params;
use plugin::Pluggable; // get_ref

use common;
use db;
use tmpl::add::tmpl_add;
use util::get_double;
use util::get_i64;
use util::get_ts;

pub fn handle_add(request: &mut Request) -> IronResult<Response> {
    let mut conn = itry!(common::get_pooled_db_connection(request));
    let account_id = match common::get_session_account_id(&mut conn, request) {
        Some(account_id) => account_id,
        None => return Ok(itry!(common::redirect(request, "new-session"))),
    };
    let resp_content_type = "text/html".parse::<Mime>().unwrap();
    let empty_btree_map = BTreeMap::new();
    let empty_hash_map = HashMap::new();
    let bank_accounts = itry!(db::get_bank_accounts(&mut conn, account_id));
    let resp_html = tmpl_add("Add", &bank_accounts, &empty_btree_map, &empty_hash_map).into_string();
    Ok(iron::response::Response::with((resp_content_type, iron::status::Ok, resp_html)))
}

pub fn handle_post_add(request: &mut Request) -> IronResult<Response> {
    let mut conn = itry!(common::get_pooled_db_connection(request));
    let account_id = match common::get_session_account_id(&mut conn, request) {
        Some(id) => id,
        None => return Ok(Response::with(iron::status::Forbidden)),
    };
    let (values, r_bank_account, r_ts, r_amount) = {
        let params = request.get_ref::<Params>().unwrap();
        (params.to_strict_map::<String>().unwrap(),
         get_i64(params, "bank_account"),
         get_ts(params, "ts"),
         get_double(params, "amount"))
    };
    match (r_bank_account, r_amount, r_ts) {
        (Ok(ref bank_account), Ok(ref amount), Ok(ref ts)) => {
            debug!("Inserting: account: {:?}, amount: {:?}, ts: {:?}",
                   bank_account,
                   amount,
                   ts);
            let amount_str = format!("{}", amount);
            itry!(db::insert_entry(&mut conn, &account_id, bank_account, ts, &amount_str));
            // let this_url = request.url.clone().into_generic_url();
            // let to_url = this_url.join("..").unwrap();
            // let to_iron_url = Url::from_generic_url(to_url).unwrap();
            // Ok(Response::with((iron::status::Found, modifiers::Redirect(to_iron_url))))
            Ok(itry!(common::redirect(request, ".")))
        }
        errs => {
            let (r_acc, r_amt, r_ts) = errs;
            let mut errors: HashMap<&str, String> = HashMap::new();
            if let Err(e) = r_acc {
                errors.insert("bank_account", e.to_string());
            }
            if let Err(e) = r_amt {
                println!("e: {}", e);
                errors.insert("amount", e.to_string());
            }
            if let Err(e) = r_ts {
                println!("e: {}", e);
                errors.insert("ts", e.to_string());
            }
            let h = tmpl_add("Add", &Vec::new(), &values, &errors).into_string();
            let ct = "text/html".parse::<Mime>().unwrap();
            Ok(Response::with((iron::status::Ok, ct, h)))
        }
    }
}
