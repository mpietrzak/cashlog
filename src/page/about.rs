
use iron;

use tmpl;

pub fn handle_about(_: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let content = tmpl::about::tmpl_about();
    Ok(iron::Response::with((iron::status::Ok, content)))
}
