
use iron::mime::Mime;
use iron::prelude::*;
use iron;
use hyper;

use model::Entry;
use common;
use db;
use model;
use tmpl;


pub fn handle_main(req: &mut Request) -> IronResult<Response> {
    let mut conn = db::connect();
    let o_account_id = common::get_session_account_id(&mut conn, req);
    if let Some(account_id) = o_account_id {
        let entries: Vec<Entry> = itry!(db::get_entries(&mut conn, account_id));
        let resp_html = tmpl::main::tmpl_main("Main", &entries);
        let resp_content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((resp_content_type, iron::status::Ok, resp_html)))
    } else {
        let o_conf = req.extensions.get::<model::Config>();
        let o_base_url: Option<String> = o_conf.map_or(None, |c| c.base_url.clone());
        let target_url: iron::Url = if let Some(conf_base_url) = o_base_url {
            let base_url = hyper::Url::parse(&conf_base_url).unwrap();
            let new_url = base_url.join("/new-session").unwrap();
            let new_iron_url = iron::Url::from_generic_url(new_url).unwrap();
            new_iron_url
        } else {
            let req_url = req.url.clone();
            let new_url = req_url.into_generic_url().join("/new-session").unwrap();
            let new_iron_url = iron::Url::from_generic_url(new_url).unwrap();
            new_iron_url
        };
        // let target_url = iron::Url::parse("https://www.google.com/").unwrap();
        Ok(Response::with((iron::status::Found, iron::modifiers::Redirect(target_url))))
    }
}
