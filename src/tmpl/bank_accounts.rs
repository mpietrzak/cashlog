use maud::html;
use maud::Markup;

use crate::model::BankAccountInfo;
use crate::tmpl::common::tmpl_base;
use crate::util;

pub fn tmpl_bank_accounts(bank_accounts: &Vec<BankAccountInfo>) -> Markup {
    let content = html! {
        p style="font-size: small" {
            "[ "
            a href="add-bank-account" {
                "Add Bank Account"
            }
            " ]"
        }
        table class="data" {
            thead {
                tr {
                    th {
                        "account"
                    }
                    th colspan="2" {
                        "amount"
                    }
                    th {
                        "updated"
                    }
                    th /
                }
            }
            tbody {
                @for bank_account_info in bank_accounts {
                    tr {
                        td {
                            (bank_account_info.bank_account)
                        }
                        td {
                            (bank_account_info.amount)
                        }
                        td {
                            (bank_account_info.currency)
                        }
                        td {
                            (util::format_ts(bank_account_info.ts))
                        }
                        td {
                            a href=(format!("graph?account={}", bank_account_info.bank_account)) {
                                "graph"
                            }
                        }
                    }
                }
            }
        }
    };
    let title = "Accounts";
    tmpl_base(title, content)
}
