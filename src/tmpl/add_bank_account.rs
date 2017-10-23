
use std::collections::HashMap;

use maud::Markup;
use maud::html;

use tmpl::common::tmpl_base;

pub fn add_bank_account(values: HashMap<String, String>, errors: HashMap<String, String>) -> Markup {
    let content = html! {
        form method="post" {
            table class="form" {
                tbody {
                    tr {
                        td label "Name:"
                        td input
                            name="name"
                            type="text"
                            value=(values.get("name").unwrap_or(&String::from("")))
                            /
                        td class="error" (errors.get("name").unwrap_or(&String::from("")))
                    }
                    tr {
                        td label "Currency:"
                        td input
                            name="currency"
                            type="text"
                            value=(values.get("currency").unwrap_or(&String::from("")))
                            /
                        td class="error" (errors.get("currency").unwrap_or(&String::from("")))
                    }
                    tr {
                        td colspan="2" align="right" {
                            button "Add"
                        }
                    }
                }
            }
        }
    };
    tmpl_base("Add Bank Account", content)
}
