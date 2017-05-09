
use maud;
use time;

use model::EntryInfo;
use tmpl::common::tmpl_base;

pub fn tmpl_graph(entries: &Vec<EntryInfo>) -> maud::Markup {
    let d: Vec<(String, f64)> = entries.iter().map(|e| {
            let t = time::strftime("%Y-%m-%d %H:%M:%S", &time::at_utc(e.ts)).unwrap();
            let a = e.amount.parse().unwrap();
            (t, a)
        }).collect();
    let j = json!(d);
    let data_js = format!("<script>var data_raw = {};</script>", j);
    let chart_js = "
        <script type=\"text/javascript\" src=\"https://www.gstatic.com/charts/loader.js\"></script>
        <script type=\"text/javascript\">
            google.charts.load('current', {'packages':['corechart']});
            google.charts.setOnLoadCallback(drawChart);
            function drawChart() {
                var data_table = new google.visualization.DataTable();
                data_table.addColumn('datetime', 'TS');
                data_table.addColumn('number', 'Amount');
                for(var i = 0; i < data_raw.length; ++i) {
                    var date_str = data_raw[i][0];
                    var amount = data_raw[i][1];
                    var date_parts = date_str.split(/[^0-9]/);
                    var date = new Date (
                        date_parts[0], date_parts[1]-1, date_parts[2],
                        date_parts[3], date_parts[4], date_parts[5]);
                    var row = [date, amount];
                    data_table.addRow(row);
                }
                var options = {
                    title: 'Amount over Time',
                    legend: { position: 'bottom' }
                };
                var chart = new google.visualization.LineChart(document.getElementById('chart'));
                chart.draw(data_table, options);
            }
        </script>
        ";
    let c = html! {
        (maud::PreEscaped(data_js))
        (maud::PreEscaped(chart_js))
        div id="chart" /
    };
    tmpl_base("Graph", c)
}
