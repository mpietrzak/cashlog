extern crate iron;
extern crate params;
extern crate postgres;
#[macro_use]
extern crate router;
extern crate time;

use iron::prelude::*;
use iron::mime::Mime;
use postgres::{Connection, SslMode};
use postgres::rows::Row;

use model::Entry;
use tmpl::tmpl_add;
use tmpl::tmpl_main;
use util::get_double;
use util::get_str;
use util::get_ts;

mod model;
mod tmpl;
mod util;

fn connect() -> Connection {
    Connection::connect("postgres://cashlog@localhost/cashlog", SslMode::None).unwrap()
}

fn row_to_entry(row: Row) -> Entry {
    Entry {
        id: row.get(0),
        amount: row.get(1),
        currency: row.get(2),
        ts: row.get(3)
    }
}

fn handle_main(_: &mut Request) -> IronResult<Response> {
    let conn = connect();
    let sql = "\
        SELECT id, amount::text, currency, ts
        FROM entry";
    let rows = conn.query(sql, &[]).unwrap();
    let entries: Vec<Entry> = rows.iter().map(row_to_entry).collect();
    let resp_html = tmpl_main("test", &entries);
    let resp_content_type = "text/html".parse::<Mime>().unwrap();
    Ok(Response::with((resp_content_type, iron::status::Ok, resp_html)))
}

fn handle_add(_: &mut Request) -> IronResult<Response> {
    let resp_content_type = "text/html".parse::<Mime>().unwrap();
    let resp_html = tmpl_add("Add");
    Ok(Response::with((resp_content_type, iron::status::Ok, resp_html)))
}

fn handle_post_add(request: &mut Request) -> IronResult<Response> {
    use params::{Params};
    let map = request.get_ref::<Params>().unwrap();
    let r_ts = get_ts(map, "ts");
    let r_amount = get_double(map, "amount");
    let r_currency = get_str(map, "currency");
    match (r_amount, r_currency, r_ts) {
        (Ok(ref amount), Ok(ref currency), Ok(ref ts)) => {
            println!("amount: {:?}, currency: {:?}, ts: {:?}", amount, currency, ts);
            let conn = connect();
            let sql = "
                insert into entry (id, amount, currency, ts)
                values (nextval('entry_seq'), $1::text::numeric, $2, $3)";
            let amount_str = format!("{}", amount);
            let r = conn.execute(sql, &[&amount_str, currency, ts]);
            println!("Result: {:?}", r);
        }
        errs => {
            println!("Something not ok with params: {:?}", errs);
        }
    }
    Ok(Response::with((iron::status::Ok, "ok")))
}

fn main() {
    let r = router!(
        get "/" => handle_main,
        get "/add" => handle_add,
        post "/add" => handle_post_add
    );
    Iron::new(r).http("localhost:14080").unwrap();
}
