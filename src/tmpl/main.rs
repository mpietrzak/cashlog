
use model::Entry;
use maud;
use time;

use tmpl::common::tmpl_base;

fn format_ts(t: time::Timespec) -> String {
    let tm = time::at_utc(t);
    let fmt = "%Y-%m-%d %H:%M:%S";
    match time::strftime(fmt, &tm) {
        Ok(s) => s,
        Err(_) => {
            // Both fmt and t should be safe correct values here,
            // so we warn if something is not right.
            warn!("Failed to format {:?} to \"{}\"",
                t,
                fmt);
            String::from("<invalid time>")
        }
    }
}

pub fn tmpl_main(title: &str, entries: &Vec<Entry>) -> maud::Markup {
    // let entries_html = tmpl_entries(entries);
    // tmpl_base(title, &entries_html)
    let content = html! {
        table class="data" {
            thead {
                tr {
                    th "ts"
                    th "account"
                    th "amount"
                    th "currency"
                }
            }
            tbody {
                @for entry in entries {
                    tr {
                        td (format_ts(entry.ts))
                        td (entry.bank_account)
                        td (entry.amount)
                        td (entry.currency)
                    }
                }
            }
        }
    };
    tmpl_base(title, content)
}
