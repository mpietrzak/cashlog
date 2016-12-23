
use maud::Markup;
use maud::PreEscaped;

pub fn tmpl_base(title: &str, content: Markup) -> Markup {
    html! {
        (PreEscaped("<!DOCTYPE html>"))
        html {
            (tmpl_head(title))
            body {
                (tmpl_menu(true))
                div class="content"  {
                    (content)
                }
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

                /* data table */

                table.data {
                    border-collapse: collapse;
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
                }"
            ))
        }
    }
}

fn tmpl_head(title: &str) -> Markup {
    html! {
        head {
            title (title)
            meta name="viewport" content="width=device-width, initial-scale=1" /
            (tmpl_css())
        }
    }
}

fn tmpl_menu(logged_in: bool) -> Markup {
    html! {
        div.menu {
            div.menu-item { "[ " a href="/" "Table" " ]" }
            div.menu-item { "[ " a href="/add" "Add" " ]" }
            div.menu-spacer ""
            @if logged_in {
                div.menu-item { "[ " a href="/profile" "Profile" " ]"}
                div.menu-item { "[ " a href="/logout" "Logout" " ]"}
            } @else {
                div.menu-item { "[ " a href="/new-session" "Login" " ]"}
            }
        }
    }
}
