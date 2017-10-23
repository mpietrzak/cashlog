
use maud::Markup;
use maud::html;
use std::fmt::Display;
use time;

use model::BankAccount;
use std::collections::BTreeMap;
use std::collections::HashMap;
use tmpl::common;

fn geth(oh: &HashMap<&str, String>, key: &str) -> String {
    let ov = oh.get(key);
    match ov {
        Some(v) => v.clone(),
        None => String::from(""),
    }
}

fn getb(ob: &BTreeMap<String, String>, key: &str, default: &str) -> String {
    let ov = ob.get(key);
    match ov {
        Some(v) => v.clone(),
        None => default.to_string(),
    }
}

fn select_field<N: Display>(
    values: &BTreeMap<String, String>,
    errors: &HashMap<&str, String>,
    label: &str,
    key: &str,
    options: &[(N, &String)],
) -> Markup {
    html! {
        tr {
            td (label)
            td {
                select name=(key) {
                    @for &(ref k, ref n) in options {
                        option value=(k) (n)
                    }
                }
            }
            td class="error" (geth(errors, key))
        }
    }
}

fn simple_field(
    values: &BTreeMap<String, String>,
    errors: &HashMap<&str, String>,
    title: &str,
    key: &str,
    default: &str,
) -> Markup {
    html! {
        tr {
            td (title)
            td input type="text" name=(key) value=(getb(values, key, default))
            td class="error" (geth(errors, key))
        }
    }
}

pub fn tmpl_add(
    title: &str,
    bank_accounts: &Vec<BankAccount>,
    values: &BTreeMap<String, String>,
    errors: &HashMap<&str, String>,
) -> Markup {
    let now = time::strftime("%Y-%m-%d %H:%M:%S", &time::now_utc()).unwrap();
    let bank_account_select_options: Vec<(i64, &String)> = bank_accounts.iter().map(|ba| (ba.id, &ba.name)).collect();
    let form = html! {
        form method="post" {
            table class="form" {
                tbody {
                    (select_field(
                        values,
                        errors,
                        "Bank Account",
                        "bank_account",
                        bank_account_select_options.as_slice()))
                    (simple_field(values, errors, "Date/Time:", "ts", &now))
                    (simple_field(values, errors, "Amount:", "amount", ""))
                    tr {
                        td colspan="3" align="right" {
                            button type="submit" "Ok"
                        }
                    }
                }
            }
        }
    };
    common::tmpl_base(title, form)
}
