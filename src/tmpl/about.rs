use maud;
use maud::html;

use crate::tmpl::common::tmpl_base;

pub fn tmpl_about() -> maud::Markup {
    let content = html! {
        h1 {
            "About"
        }
        p {
            "CashLog is a mini finance tracking app."
        }
        p {
            "CashLog uses following free or open source software and resources:"
        }
        ul {
            li {
                a href="http://fontawesome.io/" {
                    "Font Awesome"
                }
            }
            li {
                a href="https://www.rust-lang.org/" {
                    "Rust"
                }
            }
        }
    };
    tmpl_base("About", content)
}
