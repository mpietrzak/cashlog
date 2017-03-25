
use maud;

use model::EntryInfo;
use tmpl::common::tmpl_base;
use util::format_ts;

pub fn tmpl_main(title: &str, entries: &Vec<EntryInfo>) -> maud::Markup {
    // let entries_html = tmpl_entries(entries);
    // tmpl_base(title, &entries_html)
    let content = html! {
        p style="font-size: small" {
            "[ " a href="add" "Add" " ]"
        }
        table class="data" {
            thead {
                tr {
                    th "ts"
                    th "account"
                    th colspan="2" "amount"
                    th ""
                }
            }
            tbody {
                @for entry in entries {
                    tr {
                        td (format_ts(entry.ts))
                        td (entry.bank_account)
                        td (entry.amount)
                        td (entry.currency)
                        td a href=(format!("delete?id={}", entry.id)) "delete"
                    }
                }
            }
        }
    };
    tmpl_base(title, content)
}
