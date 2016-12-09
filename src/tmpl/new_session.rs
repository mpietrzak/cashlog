
use maud::Markup;

use tmpl::common::tmpl_base;

pub fn tmpl_new_session() -> Markup {
    let content = html! {
        p "Give me your e-mail and I'll send you the login link."
        form method="post" {
            table.form {
                tbody {
                    tr {
                        td input type="text" name="email" /
                    }
                    tr {
                        td align="right" {
                            button type="submit" "Ok"
                        }
                    }
                }
            }
        }
    };
    tmpl_base("New Session", content)
}

pub fn tmpl_new_session_email_sent() -> Markup {
    let content = html! {
        p "Email sent."
    };
    tmpl_base("New Session", content)
}

pub fn tmpl_new_session_result(success: bool) -> Markup {
    let title = "New Session";
    let text = match success {
        true => "Login successful",
        false => "Login failed"
    };
    tmpl_base(title, html! {
        p (text)
    })
}