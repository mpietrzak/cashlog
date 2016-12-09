
use maud::Markup;

use tmpl::common;
use std::collections::BTreeMap;
use std::collections::HashMap;

fn geth(oh: &HashMap<&str, String>, key: &str) -> String {
    let ov = oh.get(key);
    match ov {
        Some(v) => v.clone(),
        None => String::from("")
    }
}

fn getb(ob: &BTreeMap<String, String>, key: &str) -> String {
    let ov = ob.get(key);
    match ov {
        Some(v) => v.clone(),
        None => String::from("")
    }
}

fn simple_field(
    values: &BTreeMap<String, String>,
    errors: &HashMap<&str, String>,
    title: &str,
    key: &str) -> Markup {
    html! {
        tr {
            td (title)
            td input type="text" name=(key) value=(getb(values, key))
            td class="error" (geth(errors, key))
        }
    }
}

pub fn tmpl_add(
        title: &str,
        values: &BTreeMap<String, String>,
        errors: &HashMap<&str, String>) -> Markup {
    let form = html! {
        form method="post" {
            table class="form" {
                tbody {
                    (simple_field(values, errors, "Account:", "account"))
                    (simple_field(values, errors, "Date/Time:", "ts"))
                    (simple_field(values, errors, "Amount:", "amount"))
                    (simple_field(values, errors, "Currency:", "currency"))
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
