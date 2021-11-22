use chrono;
use maud::html;
use maud::Markup;
use std::fmt::Display;

use crate::model::BankAccount;
use crate::tmpl::common;

fn select_field<N: Display>(label: &str, key: &str, options: &[(N, &String)]) -> Markup {
    html! {
        tr {
            td {
                (label)
            }
            td {
                select name=(key) {
                    @for &(ref k, ref n) in options {
                        option value=(k) {
                            (n)
                        }
                    }
                }
            }
        }
    }
}

fn simple_field(title: &str, key: &str, value: &str) -> Markup {
    html! {
        tr {
            td {
                (title)
            }
            td {
                input type="text" name=(key) value=(value) /
            }
        }
    }
}

pub fn tmpl_add(title: &str, bank_accounts: &Vec<BankAccount>) -> Markup {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let bank_account_select_options: Vec<(i64, &String)> =
        bank_accounts.iter().map(|ba| (ba.id, &ba.name)).collect();
    let form = html! {
        form method="post" {
            table class="form" {
                tbody {
                    (select_field(
                        "Bank Account",
                        "bank_account",
                        bank_account_select_options.as_slice()))
                    (simple_field("Date/Time:", "ts", &now))
                    (simple_field("Amount:", "amount", ""))
                    tr {
                        td colspan="3" align="right" {
                            button type="submit" {
                                "Ok"
                            }
                        }
                    }
                }
            }
        }
    };
    common::tmpl_base(title, form)
}
