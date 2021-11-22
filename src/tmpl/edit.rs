use maud;
use maud::html;

use crate::tmpl::common::tmpl_base;

pub struct FormData {
    pub id: i64,
    pub amount: (String, Option<String>),
}

pub fn tmpl_edit(form_data: &FormData) -> maud::Markup {
    let body = html! {
        form method="post" {
            table {
                tbody {
                    tr {
                        td {
                            label {
                                "Amount"
                            }
                        }
                        td {
                            input name="amount" type="text" value=(form_data.amount.0) /
                        }
                        td class="error" {
                            @if let Some(ref err) = form_data.amount.1 {
                                (err)
                            }
                        }
                    }
                    tr {
                        td align="right" colspan="2" {
                            input name="id" type="hidden" value=(form_data.id) /
                            button type="submit" {
                                "Save"
                            }
                        }
                    }
                }
            }
        }
    };
    tmpl_base("Edit", body)
}
