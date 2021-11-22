use chrono;
use maud;
use maud::html;

use crate::tmpl::common::tmpl_base;

pub fn tmpl_export(base_href: &str) -> maud::Markup {
    let now = chrono::Utc::now();
    let now_str = now.format("%Y%m%d%H%M%S");
    let filename = format!("cashlog-export-{}.csv", now_str);
    let href = format!("{}/export/{}", base_href, filename);
    let content = html! {
        p {
            "Download entries in CSV format: "
            a href=(href) {
                (filename)
            }
        }
    };
    tmpl_base("Export", content)
}
