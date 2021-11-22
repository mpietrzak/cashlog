use actix_web::HttpMessage;

use crate::common;
use crate::db;
use crate::tmpl;
use crate::tmpl::add_bank_account::AddBankAccountTmplData;

#[derive(Deserialize)]
pub struct AddBankAccountParams {
    pub name: String,
    pub currency: String,
}

#[derive(Deserialize)]
pub struct AddBankAccountParamsValidationResult {
    pub name_err: Option<String>,
    pub currency_err: Option<String>,
}

// /// Take the raw form values data and return validation errors, if any.
// fn validate(params: AddBankAccountParams) -> AddBankAccountParamsValidationResult {
//     let mut v = AddBankAccountParamsValidationResult {
//         name_err: None,
//         currency_err: None,
//     };
//     if params.name.trim().is_empty() {
//         v.name_err = Some("Name is required".to_string());
//     }
//     if params.currency.trim().is_empty() {
//         v.currency_err = Some("Currency is required".to_string());
//     }
//     v
// }

pub async fn handle_get_add_bank_account() -> impl actix_web::Responder {
    let content = tmpl::add_bank_account::add_bank_account(&AddBankAccountTmplData {
        name: "".into(),
        name_err: "".into(),
        curr: "".into(),
        curr_err: "".into(),
    })
    .into_string();
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(content)
}

pub async fn handle_post_add_bank_account(
    request: actix_web::HttpRequest,
    pool: actix_web::web::Data<common::DatabasePool>,
    params: actix_web::web::Form<AddBankAccountParams>,
) -> impl actix_web::Responder {
    let mut conn = pool.get().unwrap();
    let cookie = match request.cookie("session") {
        Some(c) => c,
        None => {
            return actix_web::HttpResponse::SeeOther()
                .header("Location", "new-session")
                .body("Redirecting...")
        }
    };
    let sess_key = cookie.value();
    let account_id = db::get_sess_val(&mut conn, sess_key, "account")
        .unwrap()
        .parse()
        .unwrap();
    db::insert_bank_account(&mut conn, account_id, &params.name, &params.currency).unwrap();
    actix_web::HttpResponse::SeeOther().header("Location", ".").body("Redirecting...")
}
