
use maud::Markup;

use model::BankAccountInfo;
use tmpl::common::tmpl_base;
use util;

pub fn tmpl_bank_accounts(bank_accounts: &Vec<BankAccountInfo>) -> Markup {
    let content = html! {
        table class="data" {
            tbody {
                thead {
                    tr {
                        th "account"
                        th colspan="2" "amount"
                        th "updated"
                    }
                }
                tbody {
                    @for bank_account_info in bank_accounts {
                        tr {
                            td (bank_account_info.bank_account)
                            td (bank_account_info.amount)
                            td (bank_account_info.currency)
                            td (util::format_ts(bank_account_info.ts))
                        }
                    }
                }
            }
        }
    };
    let title = "Accounts";
    tmpl_base(title, content)
}
