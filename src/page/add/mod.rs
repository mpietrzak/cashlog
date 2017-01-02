
use std::collections::BTreeMap;
use std::collections::HashMap;

use iron::mime::Mime;
use iron::prelude::*;
use iron;
use params::Params;

use common;
use db;
use tmpl::add::tmpl_add;
use util::get_double;
use util::get_str;
use util::get_ts;

pub fn handle_add(request: &mut Request) -> IronResult<Response> {
    let mut conn = itry!(common::get_pooled_db_connection(request));
    if common::get_session_account_id(&mut conn, request).is_none() {
        return Ok(itry!(common::redirect(request, "/new-session")))
    }
    let resp_content_type = "text/html".parse::<Mime>().unwrap();
    let empty_btree_map = BTreeMap::new();
    let empty_hash_map = HashMap::new();
    let resp_html = tmpl_add("Add", &empty_btree_map, &empty_hash_map);
    Ok(Response::with((resp_content_type, iron::status::Ok, resp_html)))
}

pub fn handle_post_add(request: &mut Request) -> IronResult<Response> {
    let mut conn = itry!(common::get_pooled_db_connection(request));
    let account_id = match common::get_session_account_id(&mut conn, request) {
        Some(id) => id,
        None => return Ok(Response::with(iron::status::Forbidden))
    };
    let (values, r_bank_account, r_ts, r_amount, r_currency) = {
        let params = request.get_ref::<Params>().unwrap();
        (
            params.to_strict_map::<String>().unwrap(),
            get_str(params, "account"),
            get_ts(params, "ts"),
            get_double(params, "amount"),
            get_str(params, "currency")
        )
    };
    match (r_bank_account, r_amount, r_currency, r_ts) {
        (Ok(ref bank_account), Ok(ref amount), Ok(ref currency), Ok(ref ts)) => {
            debug!("Inserting: account: {:?}, amount: {:?}, currency: {:?}, ts: {:?}",
                bank_account, amount, currency, ts);
            // let sql = "
            //     insert into entry (id, account, amount, currency, ts)
            //     values (nextval('entry_seq'), $1, $2::text::numeric, $3, $4)";
            let amount_str = format!("{}", amount);
            // let r = conn.execute(sql, &[bank_account, &amount_str, currency, ts]);
            // debug!("Insert result: {:?}", r);
            // Ok(Response::with((iron::status::Ok, "ok")))
            itry!(db::insert_entry(
                &mut conn,
                account_id,
                bank_account,
                ts,
                &amount_str,
                currency));
            // let this_url = request.url.clone().into_generic_url();
            // let to_url = this_url.join("..").unwrap();
            // let to_iron_url = Url::from_generic_url(to_url).unwrap();
            // Ok(Response::with((iron::status::Found, modifiers::Redirect(to_iron_url))))
            Ok(itry!(common::redirect(request, ".")))
        }
        errs => {
            let (r_acc, r_amt, r_cur, r_ts) = errs;
            let mut errors: HashMap<&str, String> = HashMap::new();
            if let Err(e) = r_acc {
                errors.insert("account", e.to_string());
            }
            if let Err(e) = r_amt {
                println!("e: {}", e);
                errors.insert("amount", e.to_string());
            }
            if let Err(e) = r_cur {
                println!("e: {}", e);
                errors.insert("currency", e.to_string());
            }
            if let Err(e) = r_ts {
                println!("e: {}", e);
                errors.insert("ts", e.to_string());
            }
            let h = tmpl_add("Add", &values, &errors);
            Ok(Response::with((iron::status::Ok, h)))
        }
    }
}
