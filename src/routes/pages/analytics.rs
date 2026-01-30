use fluxor::prelude::*;

use crate::components::*;

use crator::{crate_data, block_on};

const MAIN_ANALYTICS_CONTENT: &str = r####"<!-- Analytics Block -->
            <section class="analytics">
                <h1 class="analytics__title">Analytics Page</h1>
                <p>
                    Fluxor and its dependencies are fundamental to this ecosystem, empowering developers to efficiently reuse and share robust components. This promotes consistency, enhances security, and accelerates development, making the framework a reliable foundation for building scalable and maintainable applications.
                </p>

                <h2 class="analytics__subtitle">Crates Total Downloads</h2>

                <div class="analytics__chart-container">
                    <canvas id="downloadsChart" width="800" height="400"></canvas>
                </div>

                <h3 class="analytics__subtitle">Crate Health Index (CHI)</h3>

                <!-- Formula Display -->
                <div style="text-align: center; margin: 20px 0;">
                    <math xmlns="http://www.w3.org" display="block" style="font-size: 1.15rem;">
                    <mi>Crate Health Index</mi>
                    <mo>=</mo>
                    <mfrac>
                        <mi>Downloads</mi>
                        <mi>Versions</mi>
                    </mfrac>
                    </math>
                </div>

                <p>&lt; 10 (Fail), 10—99 (Poor), 100—249 (Fair), 250—499 (OK), 500—999 (Good), 1000—9999 (High), 10000+ (Top)</p>

                <div class="analytics__chart-container">
                    <canvas id="healthChart" width="800" height="400"></canvas>
                </div>

                <h4 class="analytics__subtitle">Crates Information</h4>
                <div>
                    <div class="analytics__table-wrapper">
                        <table class="analytics__table">
                            <thead>
                                <tr>
                                    <th>Crate</th>
                                    <th>Latest</th>
                                    <th>Downloads</th>
                                    <th>Versions</th>
                                    <th>CHI</th>
                                    <th>License</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td>cans</td>
                                    <td>v{{cans_latest}}</td>
                                    <td>{{cans_total_downloads}}</td>
                                    <td>{{cans_versions}}</td>
                                    <td>{{cans_health_index}}</td>
                                    <td>{{cans_license}}</td>
                                </tr>
                                <tr>
                                    <td>fluxor</td>
                                    <td>v{{fluxor_latest}}</td>
                                    <td>{{fluxor_total_downloads}}</td>
                                    <td>{{fluxor_versions}}</td>
                                    <td>{{fluxor_health_index}}</td>
                                    <td>{{fluxor_license}}</td>
                                </tr>
                                <tr>
                                    <td>mathlab</td>
                                    <td>v{{mathlab_latest}}</td>
                                    <td>{{mathlab_total_downloads}}</td>
                                    <td>{{mathlab_versions}}</td>
                                    <td>{{mathlab_health_index}}</td>
                                    <td>{{mathlab_license}}</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>

                <script>
                    window.onload = () => {
                    // 1. Bar Chart
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

                    const ctx1 = document.getElementById('downloadsChart').getContext('2d');

                    const chart = new Chart(ctx1, {
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

                    const canvas = document.getElementById('downloadsChart');

                    // Function to check if mouse is over a bar
                    function isHoveringBar(evt) {
                        const points = chart.getElementsAtEventForMode(evt, 'nearest', { intersect: true }, false);
                        return points.length > 0;
                    }

                    // Change cursor to pointer when over a bar
                    canvas.addEventListener('mousemove', function(evt) {
                        if (isHoveringBar(evt)) {
                        canvas.style.cursor = 'pointer'; // Pointer cursor
                        } else {
                        canvas.style.cursor = 'default'; // Default cursor
                        }
                    });

                    // Revert cursor when leaving chart area
                    canvas.addEventListener('mouseout', function() {
                        canvas.style.cursor = 'default';
                    });

                    // Handle click on a bar
                    canvas.addEventListener('click', function(evt) {
                        const points = chart.getElementsAtEventForMode(evt, 'nearest', { intersect: true }, false);
                        if (points.length) {
                        const index = points[0].index;
                        window.open(cratesData[index].url, '_blank');
                        }
                    });

                    // 2. Health Line Chart (Health Chart)
                    const healthData = [ 
                        { crate: "cans", health: "{{cans_health_index}}", health_level: "{{cans_health_description}}", created_at: "{{cans_created_at}}", updated_at: "{{cans_updated_at}}" },
                        { crate: "fluxor", health: "{{fluxor_health_index}}", health_level: "{{fluxor_health_description}}", created_at: "{{fluxor_created_at}}", updated_at: "{{fluxor_updated_at}}" },
                        { crate: "mathlab", health: "{{mathlab_health_index}}", health_level: "{{mathlab_health_description}}", created_at: "{{mathlab_created_at}}", updated_at: "{{mathlab_updated_at}}" }
                    ];

                    const healthMap = { "Fail": 1, "Poor": 2, "Fair": 3, "OK": 4, "Good": 5, "High": 6, "Top": 7 };
                    const dataWithNulls = [null, ...healthData.map(c => healthMap[c.health_level]), null];
                    const labels = ["", ...healthData.map(c => c.crate), ""];

                    const ctx2 = document.getElementById('healthChart').getContext('2d');
                    new Chart(ctx2, {
                        type: 'line',
                        data: {
                        labels: labels,
                        datasets: [{
                            label: 'Health',
                            data: dataWithNulls,
                            borderColor: 'rgba(255, 99, 132, 1)',
                            backgroundColor: 'rgba(255, 99, 132, 0.2)',
                            tension: 0.4,
                            pointRadius: 8
                        }]
                        },
                        options: {
                        responsive: true,
                        plugins: {
                            legend: { display: false },
                            title: { display: true, text: 'Crate Health Index', font: { size: 18 } },
                            datalabels: {
                            align: 'top', anchor: 'end', font: { weight: 'bold' },
                            formatter: (value, ctx) => {
                                if (value !== null) {
                                return healthData[ctx.dataIndex - 1].health.toString();
                                }
                                return '';
                            }
                            },
                            tooltip: {
                            callbacks: {
                                label: function(context) {
                                const index = context.dataIndex;
                                if (index > 0 && index <= healthData.length) {
                                    const crate = healthData[index - 1]; 
                                    return [
                                    ` Health: ${crate.health_level} `,
                                    ` Crate Health Index: ${crate.health} `,
                                    ` Created At: ${crate.created_at} `,
                                    ` Updated At: ${crate.updated_at} `
                                    ];
                                }
                                return null;
                                }
                            }
                            }
                        },
                        scales: {
                            y: {
                            min: 0, max: 8,
                            ticks: {
                                callback: (value) => {
                                const tickLabels = ["", "Fail", "Poor", "Fair", "OK", "Good", "High", "Top", ""];
                                return tickLabels[value] || '';
                                }
                            },
                            title: { display: false, text: 'CHI' }
                            }
                        }
                        },
                        plugins: [ChartDataLabels]
                    });
                    };

                </script>
            </section>"####;

const SOURCES: &str = r##"<link rel="stylesheet" href="/css/styles.css">
    <script defer src="/js/alpine.min.js"></script>
    {{chartjs}}
    <script src="https://cdn.jsdelivr.net/npm/chartjs-plugin-datalabels"></script>"##;

pub fn analytics_page(_req: Req, _params: Params) -> Reply {
    let cans_info = block_on(crate_data("cans")).expect("Failed to get crate info");
    let fluxor_info = block_on(crate_data("fluxor")).expect("Failed to get crate info");
    let mathlab_info = block_on(crate_data("mathlab")).expect("Failed to get crate info");

    let cans_total_downloads = cans_info.total_downloads;
    let cans_latest = cans_info.latest;
    let cans_versions = cans_info.versions;
    let cans_health_index = cans_total_downloads / cans_versions;
    let cans_license = cans_info.license;
    let cans_created_at = cans_info.created_at;
    let cans_updated_at = cans_info.updated_at;

    let fluxor_total_downloads = fluxor_info.total_downloads;
    let fluxor_latest = fluxor_info.latest;
    let fluxor_versions = fluxor_info.versions;
    let fluxor_health_index = fluxor_total_downloads / fluxor_versions;
    let fluxor_license = fluxor_info.license;
    let fluxor_created_at = fluxor_info.created_at;
    let fluxor_updated_at = fluxor_info.updated_at;

    let mathlab_total_downloads = mathlab_info.total_downloads;
    let mathlab_latest = mathlab_info.latest;
    let mathlab_versions = mathlab_info.versions;
    let mathlab_health_index = mathlab_total_downloads / mathlab_versions;
    let mathlab_license = mathlab_info.license;
    let mathlab_created_at = mathlab_info.created_at;
    let mathlab_updated_at = mathlab_info.updated_at;

    boxed(async move {
        // 1. Fail: Unacceptable, completely missed requirements.
        // 2. Poor: Well below expectations; significant shortcomings.
        // 3. Fair: Below average; meets minimum requirements but with deficiencies.
        // 4. OK (or Satisfactory/Average): Meets standard expectations; acceptable.
        // 5. Good: Above average; solid, reliable performance.
        // 6. High (or Very Good): Exceeds expectations.
        // 7. Top (or Excellent/Outstanding): Exceptional, superior performance. 

        fn get_health_description(index: u64) -> &'static str {
            match index {
                0..=9 => "Fail",
                10..=99 => "Poor",
                100..=249 => "Fair",
                250..=499 => "OK",
                500..=999 => "Good",
                1000..=9999 => "High",
                _ => "Top",
            }
        }

        let cans_health_description = get_health_description(cans_health_index);
        let fluxor_health_description = get_health_description(fluxor_health_index);
        let mathlab_health_description = get_health_description(mathlab_health_index);

        let content = layout(
            "Fluxor — analytics page",
            "Fluxor is a versatile Rust web framework designed for data science and computing science applications.",
            "async, data-science, fluxor, framework, web, analytics",
            &do_html!(SOURCES, chartjs = chart_js("4.5.1")),
            &do_html!(
                MAIN_ANALYTICS_CONTENT,
                cans_total_downloads = cans_total_downloads,
                cans_latest = cans_latest,
                cans_versions = cans_versions,
                cans_health_index = cans_health_index,
                cans_health_description = cans_health_description,
                cans_license = cans_license,
                cans_created_at = cans_created_at,
                cans_updated_at = cans_updated_at,
                fluxor_total_downloads = fluxor_total_downloads,
                fluxor_latest = fluxor_latest,
                fluxor_versions = fluxor_versions,
                fluxor_health_index = fluxor_health_index,
                fluxor_health_description = fluxor_health_description,
                fluxor_license = fluxor_license,
                fluxor_created_at = fluxor_created_at,
                fluxor_updated_at = fluxor_updated_at,
                mathlab_total_downloads = mathlab_total_downloads,
                mathlab_latest = mathlab_latest,
                mathlab_versions = mathlab_versions,
                mathlab_health_index = mathlab_health_index,
                mathlab_health_description = mathlab_health_description,
                mathlab_license = mathlab_license,
                mathlab_created_at = mathlab_created_at,
                mathlab_updated_at = mathlab_updated_at
            )
        );

        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from(content))
            .unwrap())
    })
}