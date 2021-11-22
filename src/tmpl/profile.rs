//! Template for profile page.

use maud;
use maud::html;

use crate::model;
use crate::tmpl::common::tmpl_base;
use crate::util::format_ts;

pub fn tmpl_profile(acc: &model::AccountInfo) -> maud::Markup {
    let content = html! {
        h1 {"Profile"}
        p {(format!("Profile created: {}.", format_ts(acc.created_at)))}
        p {(format!("Profile modified: {}.", format_ts(acc.modified_at)))}
        p {(format!("Emails: {:?}.", acc.emails))}
    };
    tmpl_base("Profile", content)
}
