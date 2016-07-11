
use std::vec::Vec;
use time::at_utc;
use time::strftime;

use model::Entry;

fn tmpl_css() -> String {
    String::from("
        <style>

            header {
                overflow: hidden;
                background: #222;
            }

            header a, header label {
                display: block;
                padding: 20px;
                color: #fff;
                text-decoration: none;
                line-height: 20px;
            }

            header a:hover, header label:hover { color: #aaa; }

            header label {
                float: right;
                padding: 18px 20px;
                cursor: pointer;
            }

            header label:after {
                content: \"\\2261\";
                font-size: 1.8em;
            }

            .logo {
                float: left;
                font-weight: bold;
                font-size: 1.5em;
            }

            nav {
                float: right;
                max-height: 0;
                width: 100%;
                -webkit-transition: max-height 0.3s;
                -moz-transition: max-height 0.3s;
                -o-transition: max-height 0.3s;
                transition: max-height 0.3s;
            }

            nav ul {
                margin: 0;
                padding: 0;
                padding-bottom: 10px;
            }

            nav li {
                display: block;
                text-align: center;
            }

            nav a {
                padding: 10px;
                width: 100%;
            }

            #nav { display: none; }

            #nav:checked ~ nav {
                max-height: 200px; /* This can be anything bigger than your nav height. The transition duration works with this */
            }

            @media only screen and (min-width: 700px) {

                header label { display: none; }

                nav {
                    width: auto;
                    max-height: none;
                }

                nav ul {
                    padding: 0;
                    padding-right: 10px;
                }

                nav li {
                    display: inline-block;
                    text-align: left;
                }

                header nav a {
                    display: inline-block;
                    padding: 20px 10px;
                    width: auto;
                }

            }

            .content {
                padding: 2em 3em 0;
            }

            body {
                font-family: sans-serif;
                font-size: 11pt;
            }
            table.data {
                border-collapse: collapse;
            }
            table.data > thead > tr > th {
                border: 1px solid black;
                padding: 4pt;
            }
            table.data > tbody > tr > td {
                border: 1px solid gray;
                padding: 2pt;
            }
        </style>\n
    ")
}

fn tmpl_head(title: &str) -> String {
    let style_html = tmpl_css();
    format!("\
        <head>
            <title>{}</title>
            <link rel=\"stylesheet\" href=\"http://yui.yahooapis.com/pure/0.6.0/pure-min.css\">
            {}
        </head>\n", title, style_html)
}

fn tmpl_menu() -> String {
    String::from("
        <header>
            <a class=\"logo\">CashLog</a>
            <input id=\"nav\" type=\"checkbox\">
            <label for=\"nav\"></label>
            <nav>
                <ul>
                    <li><a href=\"/\">Table</a></li>
                    <li><a href=\"/add\">Add</a></li>
                </ul>
            </nav>
        </header>
    ")
}

fn tmpl_entries(entries: &Vec<Entry>) -> String {
    let mut s = String::from("");
    s.push_str("<h2>entries</h2>\n");
    s.push_str("<table class=\"data\">\n");
    s.push_str("
        <thead>
            <tr>
                <th>amount</th>
                <th>currency</th>
                <th>timestamp</th>
            </tr>
        </thead>
    ");
    s.push_str("<tbody>\n");
    for entry in entries {
        s.push_str("<tr>\n");
        let amount = &entry.amount;
        let currency = &entry.currency;
        let ts = entry.ts;
        let tm = at_utc(ts);
        let timestring = strftime("%Y-%m-%d %H:%M:%S", &tm).unwrap_or(String::from("-"));
        s.push_str(&format!(
            "<td>{}</td><td>{}</td><td>{}</td>",
            amount,
            currency,
            timestring));
        s.push_str("</tr>\n");
    };
    s.push_str("</tbody>\n");
    s.push_str("</table>\n");
    s
}

fn tmpl_base(title: &str, content: &str) -> String {
    let head_html = tmpl_head(title);
    let body_html = format!("\
        <body>
            {}
            <div id=\"layout\" class=\"pure-g\">
                <div class=\"content pure-u-1\">
                    <div>
                        {}
                    </div>
                </div>
            </div>
        </body>
    ",
        tmpl_menu(),
        content);
    format!("<!doctype html><html>{}{}</html>", head_html, body_html)
}

pub fn tmpl_main(title: &str, entries: &Vec<Entry>) -> String {
    let entries_html = tmpl_entries(entries);
    tmpl_base(title, &entries_html)
}

pub fn tmpl_add(title: &str) -> String {
    let form = "
        <form method=\"post\">
            <table>
                <tr>
                    <td>Amount:</td>
                    <td><input type=\"text\" name=\"amount\"></td>
                </tr>
                <tr>
                    <td>Currency:</td>
                    <td><input type=\"text\" name=\"currency\"></td>
                </tr>
                <tr>
                    <td>Timestamp:</td>
                    <td><input type=\"text\" name=\"ts\"></td>
                </tr>
                <tr>
                    <td colspan=\"2\" style=\"text-align: right\">
                        <button>Ok</button>
                    </td>
                </tr>
            </table>
        </form>
    ";
    tmpl_base(title, form)
}
