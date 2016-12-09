
use maud::Markup;
use maud::PreEscaped;

pub fn tmpl_base(title: &str, content: Markup) -> Markup {
    html! {
        (tmpl_head(title))
        body {
            (tmpl_menu())
            div id="layout" class="pure-g" {
                div class="content pure-u-1" {
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

                .content {
                    margin: 2em 3em 0;
                }

                body {
                    font-family: Clear Sans, Helvetica, sans-serif;
                    font-size: 11pt;
                }

                /* menu */

                div.menu {
                    margin: 2em 3em;
                }

                div.menu > div.menu-item {
                    display: inline-block;
                    color: #2A261D;
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
            (tmpl_css())
        }
    }
}

fn tmpl_menu() -> Markup {
    html! {
        div.menu {
            div.menu-item { "[ " a href="/" "Table" " ]" }
            div.menu-item { "[ " a href="/add" "Add" " ]" }
            div.menu-item { "[ " a href="/new-session" "New Session" " ]"}
        }
    }
}
