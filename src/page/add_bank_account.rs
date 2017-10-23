
use std::collections::HashMap;

use iron;
use iron::IronResult;
use iron::Request;
use iron::Response;
use params::Params;
use params::Value;
use plugin::Pluggable; // get_ref

use common;
use db;
use tmpl;

pub fn handle_get_add_bank_account(_request: &mut Request) -> IronResult<Response> {
    let errors = HashMap::new();
    let values = HashMap::new();
    let content = tmpl::add_bank_account::add_bank_account(values, errors);
    Ok(Response::with((iron::status::Ok, content)))
}

/// Extract form values from request.
/// We might want to use something else than hashmap here some day to be
/// more type safe...
fn get_values(request: &mut Request) -> HashMap<String, String> {
    let mut values = HashMap::new();
    let params = request.get_ref::<Params>().unwrap();
    if let Some(&Value::String(ref n)) = params.get("name") {
        values.insert(String::from("name"), n.clone());
    }
    if let Some(&Value::String(ref n)) = params.get("currency") {
        values.insert(String::from("currency"), n.clone());
    }
    values
}

/// Take the raw form values data and return validation errors, if any.
fn validate(values: &HashMap<String, String>) -> HashMap<String, String> {
    let mut errors = HashMap::new();
    if values.get("name").unwrap_or(&String::from("")) == "" {
        errors.insert(String::from("name"), String::from("Name is required"));
    }
    if values.get("currency").unwrap_or(&String::from("")) == "" {
        errors.insert(
            String::from("currency"),
            String::from("Currency is required"),
        );
    }
    errors
}

pub fn handle_post_add_bank_account(request: &mut Request) -> IronResult<Response> {
    let mut conn = itry!(common::get_pooled_db_connection(request));
    let account_id = match common::get_session_account_id(&mut conn, request) {
        Some(account_id) => account_id,
        None => return Ok(itry!(common::redirect(request, "new-session"))),
    };
    let values = get_values(request);
    let errors = validate(&values);
    if errors.is_empty() {
        let ref name = values["name"];
        let ref currency = values["currency"];
        itry!(db::insert_bank_account(
            &mut conn,
            account_id,
            &name,
            &currency
        ));
        Ok(itry!(common::redirect(request, "add")))
    } else {
        // show form
        let content = tmpl::add_bank_account::add_bank_account(values, errors);
        Ok(Response::with((iron::status::Ok, content)))
    }
}
