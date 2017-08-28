
//! Template for profile page.

use maud;
use maud::html;

use tmpl::common::tmpl_base;
use model;
use util::format_ts;

pub fn tmpl_profile(acc: &model::AccountInfo) -> maud::Markup {
    let content = html! {
        h1 "Profile"
        p (format!("Profile created: {}.", format_ts(acc.created)))
        p (format!("Profile modified: {}.", format_ts(acc.modified)))
        p (format!("Emails: {:?}.", acc.emails))
    };
    tmpl_base("Profile", content)
}
