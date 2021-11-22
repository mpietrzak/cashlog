use maud::html;
use maud::Markup;

use crate::tmpl::common::tmpl_base;

pub struct AddBankAccountTmplData {
    pub name: String,
    pub name_err: String,
    pub curr: String,
    pub curr_err: String,
}

pub fn add_bank_account(data: &AddBankAccountTmplData) -> Markup {
    let content = html! {
        form method="post" {
            table class="form" {
                tbody {
                    tr {
                        td label {
                            "Name:"
                        }
                        td {
                            input
                                name="name"
                                type="text"
                                value=(data.name) /
                        }
                        td class="error" {
                            (data.name_err)
                        }
                    }
                    tr {
                        td label {
                            "Currency:"
                        }
                        td {
                            input
                                name="currency"
                                type="text"
                                value=(data.curr)
                                /
                        }
                        td class="error" {
                            (data.curr_err)
                        }
                    }
                    tr {
                        td colspan="2" align="right" {
                            button {
                                "Add"
                            }
                        }
                    }
                }
            }
        }
    };
    tmpl_base("Add Bank Account", content)
}
