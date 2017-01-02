
use maud;
use time;

use tmpl::common::tmpl_base;

pub fn tmpl_export(base_href: &str) -> maud::Markup {
    let now = time::now();
    let now_str = time::strftime("%Y%m%d%H%M%S", &now).unwrap();
    let filename = format!("cashlog-export-{}.csv", now_str);
    let href = format!("{}/export/{}", base_href, filename);
    let content = html! {
        p {
            "Download entries in CSV format: "
            a href=(href) (filename)
        }
    };
    tmpl_base("Export", content)
}
