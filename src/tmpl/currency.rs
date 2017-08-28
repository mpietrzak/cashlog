
use maud;
use maud::html;

use model::CurrencyInfo;
use tmpl::common::tmpl_base;
use util;

pub fn tmpl_currency(currency_info: Vec<CurrencyInfo>) -> maud::Markup {
    let content = html! {
        table class="data" {
            thead {
                tr {
                    th "currency"
                    th "amount"
                    th "updated"
                }
            }
            tbody {
                @for currency_info in currency_info {
                    tr {
                        td (currency_info.currency)
                        td (currency_info.amount)
                        td (util::format_ts(currency_info.ts))
                    }
                }
            }
        }
    };
    tmpl_base("Currency", content)
}
