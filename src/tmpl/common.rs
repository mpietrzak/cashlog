use maud;
use maud::html;
use maud::Markup;
use maud::PreEscaped;

pub fn tmpl_base(title: &str, content: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            (tmpl_head(title))
            body {
                (tmpl_menu(true))
                div class="content" {
                    (content)
                }
                (tmpl_foot())
            }
        }
    }
}

/// Create the footer div.
fn tmpl_foot() -> Markup {
    /*
    let mem_info = if let Ok(mem) = psutil::process::Process::current()
        .expect("Failed to get current process info")
        .memory_info()
    {
        format!("Memory usage: {}.", human_bytes(mem.rss()))
    } else {
        String::from("")
    };
    */
    let mem_info = "";
    html! {
        hr /
        div class="foot" {
            p {
                "CashLog is mini-finance tracking app."
                br /
                "CashLog is Open Source and it's written in Rust."
                br /
                "CashLog uses cookies."
                br /
                (mem_info)
            }
        }
    }
}

fn tmpl_css() -> Markup {
    html! {
        style {
            (PreEscaped(
                "
                /* general */

                /* default */
                div.content {
                    margin: 4px;
                }
                div.menu {
                    margin: 4px;
                }

                /* small */
                @media (max-width: 800px) {
                    div.content {
                        margin: 8px;
                    }
                    div.menu {
                        margin: 8px;
                    }
                }

                /* big */
                @media screen and (min-width: 800px) {
                    div.content {
                        margin: 16px;
                    }
                    div.menu {
                        margin: 16px;
                    }
                }

                body {
                    font-family: Clear Sans, Helvetica, sans-serif;
                    font-size: 16px;
                }

                a,
                a:hover,
                a:active,
                a:visited {
                    color: #CC4237;
                }

                a.active {
                    color: #A63817;
                }

                /* menu */
                div.menu {
                    display: flex;
                    flex-wrap: wrap;
                    justify-content: flex-start;
                }

                div.menu > div.menu-item {
                    color: #2A261D;
                }

                div.menu > div.menu-spacer {
                    flex-grow: 1;
                }

                div.menu > div.menu-item a,
                div.menu > div.menu-item a:hover,
                div.menu > div.menu-item a:active,
                div.menu > div.menu-item a:visited {
                    color: #CC4237;
                }

                div.menu > div.menu-item a.active {
                    color: #A63817;
                }

                /* footer */

                div.foot {
                    font-size: 70%;
                    text-align: center;
                }

                /* data table */

                table.data {
                    border-collapse: collapse;
                    font-size: 80%;
                }

                table.data > thead > tr > th {
                    border: 1px solid #2A261D;
                    background-color: #E2D7B7;
                    padding: 4pt;
                }

                table.data > tbody > tr > td {
                    border: 1px solid #665F4F;
                    padding: 2pt;
                }

                /* form */

                table.form {
                    border-collapse: collapse;
                }

                table.form > tbody > tr > td {
                    padding: 1pt 2pt 1pt 0pt;
                }

                table.form > tbody > tr > td.error {
                    font-size: 90%;
                    color: red;
                }"
            ))
        }
    }
}

fn tmpl_head(title: &str) -> Markup {
    let title = {
        let lt = title.to_lowercase();
        if !&lt.starts_with("cashlog ") {
            format!("CashLog :: {}", title)
        } else {
            title.to_string()
        }
    };
    html! {
        head {
            title {
                (title)
            }
            meta name="viewport" content="width=device-width, initial-scale=1" /
            (tmpl_css())
        }
    }
}

fn tmpl_menu(logged_in: bool) -> Markup {
    html! {
        div.menu {
            div.menu-item {
                "[ "
                a href="/" {
                    "Entries"
                }
                " ]"
            }
            div.menu-item {
                "[ "
                a href="/accounts" {
                    "Accounts"
                }
                " ]"
            }
            div.menu-item {
                "[ "
                a href="/currency" {
                    "Currency"
                }
                " ]"
            }
            div.menu-item { "[ " a href="/export" { "Export" } " ]" }
            div.menu-spacer {}
            div.menu-item { "[ " a href="/about" { "About" } " ]" }
            @if logged_in {
                div.menu-item { "[ " a href="/profile" { "Profile" } " ]"}
                div.menu-item { "[ " a href="/logout" { "Logout" } " ]"}
            } @else {
                div.menu-item { "[ " a href="/new-session" { "Login" } " ]"}
            }
        }
    }
}
