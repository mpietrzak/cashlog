use maud;
use maud::html;

use crate::tmpl::common::tmpl_base;

pub fn tmpl_logout() -> maud::Markup {
    let content = html! {
        form method="post" {
            table.form {
                tbody {
                    tr {
                        td {
                            "Are you sure?"
                        }
                    }
                    tr {
                        td align="right" {
                            button type="submit" name="confirm" value="no" {
                                "No"
                            }
                            button type="submit" name="confirm" value="yes" {
                                "Yes"
                            }
                        }
                    }
                }
            }
        }
    };
    tmpl_base("Logout", content)
}
