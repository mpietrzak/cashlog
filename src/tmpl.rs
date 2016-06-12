
use std::vec::Vec;
use time::at_utc;
use time::strftime;

use model::Entry;

fn tmpl_css() -> String {
    String::from("
        <style>
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
    format!("<head><title>{}</title>{}</head>\n", title, style_html)
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
    let body_html = format!("<body>{}</body>", content);
    format!("<html>{}{}</html>", head_html, body_html)
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
