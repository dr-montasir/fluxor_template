use fluxor::prelude::*;

use crate::components::*;

use crator::crate_data;

const MAIN_ANALYTICS_CONTENT: &str = r####"<!-- Analytics Block -->
            <section class="analytics">
                <h1>Analytics Page</h1>
                <p>
                    Fluxor and its dependencies are fundamental to this ecosystem, empowering developers to efficiently reuse and share robust components. This promotes consistency, enhances security, and accelerates development, making the framework a reliable foundation for building scalable and maintainable applications.
                </p>

                <h2>Crates Total Downloads</h2>
                <div class="analytics__chart-container">
                    <canvas id="downloadsChart" width="800" height="400"></canvas>
                </div>

                <h3>Crates Information</h3>
                <div>
                    <div class="analytics__table-wrapper">
                        <table class="analytics__table">
                        <thead>
                            <tr>
                                <th>Crate</th>
                                <th>Downloads</th>
                                <th>Latest</th>
                                <th>License</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>cans</td>
                                <td>{{cans_total_downloads}}</td>
                                <td>v{{cans_latest}}</td>
                                <td>{{cans_license}}</td>
                            </tr>
                            <tr>
                                <td>fluxor</td>
                                <td>{{fluxor_total_downloads}}</td>
                                <td>v{{fluxor_latest}}</td>
                                <td>{{fluxor_license}}</td>
                            </tr>
                            <tr>
                                <td>mathlab</td>
                                <td>{{mathlab_total_downloads}}</td>
                                <td>v{{mathlab_latest}}</td>
                                <td>{{mathlab_license}}</td>
                            </tr>
                        </tbody>
                        </table>
                    </div>
                </div>

                <script>
                    const cratesData = [
                    {
                        crate: "cans",
                        url: "https://crates.io/crates/cans",
                        downloads: {{cans_total_downloads}},
                        latest_version: "{{cans_latest}}",
                        license: "{{cans_license}}"
                    },
                    {
                        crate: "fluxor",
                        url: "https://crates.io/crates/fluxor",
                        downloads: {{fluxor_total_downloads}},
                        latest_version: "{{fluxor_latest}}",
                        license: "{{fluxor_license}}"
                    },
                    {
                        crate: "mathlab",
                        url: "https://crates.io/crates/mathlab",
                        downloads: {{mathlab_total_downloads}},
                        latest_version: "{{mathlab_latest}}",
                        license: "{{mathlab_license}}"
                    }
                    ];

                    const ctx = document.getElementById('downloadsChart').getContext('2d');

                    const chart = new Chart(ctx, {
                    type: 'bar',
                    data: {
                        labels: cratesData.map(c => c.crate),
                        datasets: [{
                        label: 'Crates Total Downloads',
                        data: cratesData.map(c => c.downloads),
                        backgroundColor: 'rgba(75, 192, 192, 0.7)',
                        }]
                    },
                    options: {
                        responsive: true,
                        onClick: (evt, elements) => {
                        if (elements.length > 0) {
                            const index = elements[0].index;
                            const crateInfo = cratesData[index];
                            window.open(crateInfo.url, '_blank');
                        }
                        },
                        plugins: {
                            datalabels: {
                                    anchor: 'end',
                                    align: 'top',
                                    formatter: (value, context) => {
                                    const index = context.dataIndex;
                                    return cratesData[index].downloads.toLocaleString(); // show number
                                },
                                font: { size: 14, weight: 'bold' },
                                color: 'black'
                            },
                            tooltip: {
                                callbacks: {
                                label: function(context) {
                                    const index = context.dataIndex;
                                    const crate = cratesData[index];
                                    return [
                                    `Crate: ${crate.crate}`,
                                    `Version: ${crate.latest_version}`,
                                    `License: ${crate.license}`,
                                    `Link: ${crate.url}`
                                    ];
                                }
                                }
                            },
                            title: {
                                display: true,
                                text: 'Crates Total Downloads'
                            }
                            },
                            scales: {
                                y: {
                                    beginAtZero: true,
                                    min: 0,
                                    ticks: {
                                    callback: function(value) {
                                        if (value === 0) return 0;
                                        if (value < 100) return value;
                                        return value.toLocaleString();
                                    }
                                    }
                                }
                            }
                        },
                        plugins: [ChartDataLabels]
                    });

                    // Detect clicks near the number labels to open links
                    document.getElementById('downloadsChart').addEventListener('click', function(evt) {
                    const points = chart.getElementsAtEventForMode(evt, 'nearest', { intersect: false }, false);
                    if (points.length) {
                        const index = points[0].index;
                        const crateInfo = cratesData[index];

                        // Get position of the clicked point
                        const meta = chart.getDatasetMeta(0);
                        const bar = meta.data[index];

                        // Get click position relative to canvas
                        const rect = chart.canvas.getBoundingClientRect();
                        const clickX = evt.clientX - rect.left;
                        const clickY = evt.clientY - rect.top;

                        // The number is rendered above the bar, roughly 15-20 px above bar.y
                        const labelYPosition = bar.y - 20;
                        const labelXPosition = bar.x;

                        // Check if click is near the label
                        if (
                        Math.abs(clickX - labelXPosition) < 30 && // horizontal range
                        clickY < bar.y && clickY > labelYPosition // vertical range
                        ) {
                        window.open(crateInfo.url, '_blank');
                        }
                    }
                    });
                </script>
            </section>"####;

const SOURCES: &str = r##"<link rel="stylesheet" href="/css/styles.css">
    <script defer src="/js/alpine.min.js"></script>
    {{chartjs}}
    <script src="https://cdn.jsdelivr.net/npm/chartjs-plugin-datalabels"></script>"##;

pub fn analytics_page(_req: Req, _params: Params) -> Reply {
    boxed(async move {
        let cans_info = crate_data("cans").await.expect("Failed to fetch cans data");
        let fluxor_info = crate_data("fluxor").await.expect("Failed to fetch fluxor data");
        let mathlab_info = crate_data("mathlab").await.expect("Failed to fetch mathlab data");

        let content = layout(
            "Fluxor — analytics page",
            "Fluxor is a versatile Rust web framework designed for data science and computing science applications.",
            "async, data-science, fluxor, framework, web, analytics",
            &do_html!(SOURCES, chartjs = chart_js("4.5.1")),
            &do_html!(
                MAIN_ANALYTICS_CONTENT,
                cans_total_downloads = cans_info.total_downloads,
                cans_latest = &cans_info.latest,
                cans_license = &cans_info.license,
                fluxor_total_downloads = fluxor_info.total_downloads,
                fluxor_latest = &fluxor_info.latest,
                fluxor_license = &fluxor_info.license,
                mathlab_total_downloads = mathlab_info.total_downloads,
                mathlab_latest = &mathlab_info.latest,
                mathlab_license = &mathlab_info.license
            )
        );

        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from(content))
            .unwrap())
    })
}