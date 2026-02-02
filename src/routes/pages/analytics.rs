use fluxor::prelude::*;
use fluxor::wtime::utc::format_utc_ts;
use crator::{crate_data, block_on};

use crate::components::*;
use crate::ds;

const MAIN_ANALYTICS_CONTENT: &str = r####"<!-- Analytics Block -->
            <section class="analytics">
                <h1 class="analytics__title">Analytics Page</h1>
                <p>
                    Fluxor and its dependencies are fundamental to this ecosystem, empowering developers to efficiently reuse and share robust components. This promotes consistency, enhances security, and accelerates development, making the framework a reliable foundation for building scalable and maintainable applications.
                </p>

                <h2 class="analytics__subtitle">Crates Total Downloads</h2>

                <div class="analytics__chart-container">
                    <canvas id="downloadsChart" class="analytics__canvas"></canvas>
                </div>

                <h3 class="analytics__subtitle">Crate Health Index (CHI)</h3>

                <!-- Formula Display -->
                <div "class="analytics__formula">
                    <math xmlns="http://www.w3.org" display="block" class="analytics__formula-math">
                        <mi>Crate Health Index</mi>
                        <mo>=</mo>
                        <mfrac>
                            <mi>Downloads</mi>
                            <mi>Versions</mi>
                        </mfrac>
                    </math>
                </div>

                <p class="class="analytics__chi-scale"">
                    &lt; 10 (Fail), 10—99 (Poor), 100—249 (Fair), 250—499 (OK), 500—999 (Good), 1000—9999 (High), 10000+ (Top)
                </p>

                <div class="analytics__chart-container">
                    <canvas id="healthChart" class="analytics__canvas"></canvas>
                </div>

                <h3 class="analytics__subtitle">Crates Information</h3>
                
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
                
                <h3 class="analytics__subtitle">Crate Ecosystem Growth Dashboard</h3>
                <p>
                    This example demonstrates the application of the <b>Fluxor framework</b> within a data science context. Please note that simulated results may deviate from real-world data, as several external variables have been excluded from this model.
                </p>

                <div class="analytics__chart-container">
                    <canvas id="crateBubbleChart" class="analytics__canvas"></canvas>
                </div>

                <!-- Controls -->
                <div class="analytics__buttons-container">
                    <button class="btn btn--simulate" onclick="applyDynamicScenario(true)">
                        🚀 Simulate +2 Years Growth
                    </button>
                    
                    <button class="btn btn--reset" onclick="applyDynamicScenario(false)">
                        🔄 Reset to Baseline
                    </button>
                </div>

                <!-- Data Science Comparison Table -->
                <div class="analytics__table-wrapper">
                    <table class="analytics__table">
                        <thead>
                            <tr>
                                <th>Metric</th>
                                <th>
                                    Mathlab
                                    <br>
                                    (<span id="base-age-mathlab">{{mathlab_age_in_years_fix_1}}</span>y base)
                                </th>
                                <th>
                                    Cans
                                    <br>
                                    (<span id="base-age-cans">{{cans_age_in_years_fix_1}}</span>y base)
                                </th>
                                <th>
                                    Fluxor
                                    <br>
                                    (<span id="base-age-fluxor">{{fluxor_age_in_years_fix_1}}</span>y base)
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><span class="modeLabel">Current</span> Downloads</td>
                                <td id="dl-mathlab">{{mathlab_total_downloads}}</td>
                                <td id="dl-cans">{{cans_total_downloads}}</td>
                                <td id="dl-fluxor">{{fluxor_total_downloads}}</td>
                            </tr>
                            <tr>
                                <td><span class="modeLabel">Current</span> Versions</td>
                                <td id="ver-mathlab">{{mathlab_versions}}</td>
                                <td id="ver-cans">{{cans_versions}}</td>
                                <td id="ver-fluxor">{{fluxor_versions}}</td>
                            </tr>
                            <tr>
                                <td><span class="modeLabel">Current</span> Age (Years)</td>
                                <td id="age-mathlab">{{mathlab_age_in_years_fix_1}}</td>
                                <td id="age-cans">{{cans_age_in_years_fix_1}}</td>
                                <td id="age-fluxor">{{fluxor_age_in_years_fix_1}}</td>
                            </tr>
                            <tr>
                                <td><span class="modeLabel">Current</span> CHI Health</td>
                                <td id="chi-mathlab">{{mathlab_health_index}}</td>
                                <td id="chi-cans">{{cans_health_index}}</td>
                                <td id="chi-fluxor">{{fluxor_health_index}}</td>
                            </tr>
                        </tbody>
                    </table>
                <div>

            </section>
            
            <section class="analytics__report-section">
                <!-- Documentation Section -->
                <div class="analytics__doc">

                    <h3 class="analytics__doc-title analytics__doc-title--center">Methodology & Growth Projection</h3>         

                    <div class="analytics__list">
                        <p class="analytics__list-text">
                            The simulation utilizes a <b>Temporal Velocity Model</b> to project performance over a time delta (2 years).
                        </p>
                        <div "class="analytics__formula">
                            <math xmlns="http://www.w3.org" display="block" class="analytics__formula-math">
                                <msub>
                                    <mi>T</mi><mi>Δ</mi>
                                </msub>
                                <mo>=</mo>
                                <mn>730</mn>
                                <mspace width="0.3em"></mspace>
                                <mi>days</mi>
                            </math>
                        </div>
                    </div>

                    <div class="analytics__methods">
                        <h3 class="analytics__doc-title">1. Methodology & Mathematical Formulas</h3>
                        <div class="analytics__method">
                            <div class="analytics__method-description">
                                <b>Download Velocity:</b> Projected downloads use a Market Success Factor (<i>F</i>).
                            </div>
                            <div "class="analytics__formula">
                                <math xmlns="http://www.w3.org" display="block" class="analytics__formula-math">
                                    <msub><mi>DL</mi><mi>sim</mi></msub>
                                    <mo>=</mo>
                                    <msub><mi>DL</mi><mi>base</mi></msub>
                                    <mo>+</mo>
                                    <mfenced>
                                        <mfrac>
                                        <msub><mi>DL</mi><mi>base</mi></msub>
                                        <msub><mi>Age</mi><mi>days</mi></msub>
                                        </mfrac>
                                        <mo>×</mo>
                                        <msub><mi>T</mi><mi>Δ</mi></msub>
                                        <mo>×</mo>
                                        <mi>F</mi>
                                    </mfenced>
                                </math>
                            </div>
                        </div>
                        <div class="analytics__method">
                            <div class="analytics__method-description">
                                <b>Maturity Projection:</b> Versions increment based on specific activity tiers: 3/yr (Stable), 5/yr (Maintenance), or Density-based (Active).
                            </div>
                            <div "class="analytics__formula">
                                <math xmlns="http://www.w3.org" display="block" class="analytics__formula-math">
                                    <msub><mi>Ver</mi><mi>sim</mi></msub>
                                    <mo>=</mo>
                                    <msub><mi>Ver</mi><mi>base</mi></msub>
                                    <mo>+</mo>
                                    <mfenced>
                                        <msub><mi>Versions</mi><mi>year</mi></msub>
                                        <mo>×</mo>
                                        <mn>2</mn>
                                    </mfenced>
                                </math>
                            </div>
                        </div>
                        <div class="analytics__method">
                            <div class="analytics__method-description">
                                <b>CHI Stability:</b> Project Health is simulated as a function of download density, where higher adoption per day signifies a more stable community foundation.
                            <div> 
                            <div "class="analytics__formula">
                                <math xmlns="http://www.w3.org" display="block" class="analytics__formula-math">
                                    <msub><mi>CHI</mi><mi>sim</mi></msub>
                                    <mo>=</mo>
                                    <msub><mi>CHI</mi><mi>base</mi></msub>
                                    <mo>+</mo>
                                    <mfenced>
                                    <mrow>
                                        <mfrac>
                                        <msub><mi>DL</mi><mi>sim</mi></msub>
                                        <msub><mi>Age</mi><mi>sim</mi></msub>
                                        </mfrac>
                                        <mo>×</mo>
                                        <mn>20</mn>
                                    </mrow>
                                    </mfenced>
                                </math>
                            </div>
                        </div>
                    </div>

                    <h3 class="analytics__doc-title">2. Analytical Growth Report</h3>

                    <div class="analytics__growth-report">
                        <p class="analytics__growth-summary">
                            <b>Executive Summary:</b> By applying a <span id="report-years">2</span>-year temporal leap, the dashboard visualizes the "Winner-Takes-Most" effect. As <b>Mathlab</b> reaches a projected age of <span id="report-age-mathlab">7.0</span> years, its health index surges to <span id="report-chi-mathlab">2,866</span>.
                        </p>
                        
                        <p class="analytics__growth-details">
                            <b>Dynamic Observations:</b>
                            <ul class="analytics__observations">
                                <li class="analytics__observation">
                                    <b>Dominance:</b> Mathlab's <span id="report-dl-mathlab">235,931</span> downloads demonstrate established trust scalability despite low version activity.
                                </li>
                                <li class="analytics__observation">
                                    <b>Uplift:</b> <b>Cans</b> and <b>Fluxor</b> transition to CHI levels of <span id="report-chi-cans">1,003</span> and <span id="report-chi-fluxor">444</span> respectively.
                                </li>
                                <li class="analytics__observation">
                                    <b>Stability:</b> This confirms project health is driven by the density of usage relative to the age of the crate.
                                </li>
                            </ul>
                        </p>
                    </div>
                </div>
            </section>

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
                        backgroundColor: 'rgba(97, 218, 251, 0.7)',
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
                            borderColor: 'rgba(217, 116, 11, 1)',
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
                            formatter: (value, ctx2) => {
                                if (value !== null) {
                                return healthData[ctx2.dataIndex - 1].health.toString();
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

            <!-- 3. Temporal Velocity Model (Stability CHI Health Chart) -->
            <script>
                const ctx = document.getElementById('crateBubbleChart').getContext('2d');
                const scaleRadius = (val) => Math.sqrt(val) / 15;
                
                const baseData = [
                    { id: 'cans',    dl: {{cans_total_downloads}}, chi: {{cans_health_index}}, ver: {{cans_versions}}, ageDays: {{cans_age_in_days}}, color: 'rgba(97, 218, 251, 0.6)' },
                    { id: 'fluxor',  dl: {{fluxor_total_downloads}}, chi: {{fluxor_health_index}}, ver: {{fluxor_versions}}, ageDays: {{fluxor_age_in_days}}, color: 'rgba(217, 116, 11, 0.6)' },
                    { id: 'mathlab', dl: {{mathlab_total_downloads}}, chi: {{mathlab_health_index}}, ver: {{mathlab_versions}}, ageDays: {{mathlab_age_in_days}}, color: 'rgba(75, 192, 192, 0.6)' }
                ];
                
                const bubbleChart = new Chart(ctx, {
                    type: 'bubble',
                    data: {
                    datasets: baseData.map(item => ({
                        label: item.id,
                        data: [{ x: item.ver, y: item.chi, r: scaleRadius(item.dl) }],
                        backgroundColor: item.color,
                        borderColor: item.color.replace('0.6', '1'),
                        borderWidth: 2
                    }))
                    },
                    options: {
                    responsive: true,
                    animation: { duration: 1500, easing: 'easeOutQuart' },
                    scales: {
                        x: { title: { display: true, text: 'Maturity (Versions)' }, min: 0, suggestedMax: 150 },
                        y: { type: 'linear', title: { display: true, text: 'Stability (CHI Health)' }, beginAtZero: true, grace: '15%' }
                    }
                    }
                });
                
                function applyDynamicScenario(isGrowth) {
                    const yearsDelta = 2;
                    const daysToAdd = yearsDelta * 365;
                    const modeText = isGrowth ? 'Simulated' : 'Current';
                    const accentColor = isGrowth ? '#61dafb' : '#444';
                
                    document.querySelectorAll('.modeLabel').forEach(el => {
                    el.innerText = modeText;
                    el.style.color = accentColor;
                    });
                
                    bubbleChart.data.datasets.forEach((dataset, index) => {
                    const base = baseData[index];
                    let newDl, newVer, newChi, newAgeDays;
                
                    if (isGrowth) {
                        newAgeDays = base.ageDays + daysToAdd;
                        const dlVelocity = base.dl / base.ageDays;
                        const factor = (base.id === 'mathlab') ? 8 : 3;
                
                        // Download Growth Logic
                        newDl = base.dl + (dlVelocity * daysToAdd * factor);
                
                        // Version Growth Logic (Updated as requested)
                        if (base.id === 'mathlab') {
                            newVer = base.ver + (3 * yearsDelta); // Fixed 3 versions per year
                        } else if (base.id === 'cans') {
                            newVer = base.ver + (5 * yearsDelta); // Fixed 5 versions per year
                        } else {
                            // Fluxor remains highly active based on its density
                            const verVelocity = base.ver / base.ageDays;
                            newVer = Math.round(base.ver + (verVelocity * daysToAdd * 1.5));
                        }
                        
                        newChi = base.chi + ((newDl / newAgeDays) * 20);
                    } else {
                        newDl = base.dl; newVer = base.ver; newChi = base.chi; newAgeDays = base.ageDays;
                    }
                
                    dataset.data[0].x = newVer;
                    dataset.data[0].y = newChi;
                    dataset.data[0].r = scaleRadius(newDl);
                
                    const updateEl = (id, val) => {
                        const el = document.getElementById(`${id}-${base.id}`);
                        if(el) {
                        el.innerText = val;
                        el.style.color = accentColor;
                        el.style.fontWeight = isGrowth ? 'bold' : 'normal';
                        }
                    };
                
                    updateEl('dl', Math.round(newDl).toLocaleString());
                    updateEl('ver', newVer.toLocaleString());
                    updateEl('age', (newAgeDays / 365).toFixed(1));
                    updateEl('chi', Math.round(newChi).toLocaleString());
                
                    if(document.getElementById(`report-age-${base.id}`)) {
                        document.getElementById(`report-age-${base.id}`).innerText = (newAgeDays / 365).toFixed(1);
                    }
                    if(document.getElementById(`report-chi-${base.id}`)) {
                        document.getElementById(`report-chi-${base.id}`).innerText = Math.round(newChi).toLocaleString();
                    }
                    if(document.getElementById(`report-dl-${base.id}`)) {
                        document.getElementById(`report-dl-${base.id}`).innerText = Math.round(newDl).toLocaleString();
                    }
                    });
                
                    bubbleChart.update();
                }
                
                applyDynamicScenario(false);
            </script>"####;

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
    let cans_health_description = ds::get_health_description(cans_health_index);
    let cans_license = cans_info.license;
    let cans_created_at = cans_info.created_at;
    let cans_updated_at = cans_info.updated_at;
    let cans_age_in_days = ds::get_days(&format_utc_ts().to_string(), &cans_created_at);
    let cans_age_in_years = ds::get_days(&format_utc_ts().to_string(), &cans_created_at) as f64 / 365 as f64;
    let cans_age_in_years_fix_1 = math::fix(cans_age_in_years, 1);

    let fluxor_total_downloads = fluxor_info.total_downloads;
    let fluxor_latest = fluxor_info.latest;
    let fluxor_versions = fluxor_info.versions;
    let fluxor_health_index = fluxor_total_downloads / fluxor_versions;
    let fluxor_health_description = ds::get_health_description(fluxor_health_index);
    let fluxor_license = fluxor_info.license;
    let fluxor_created_at = fluxor_info.created_at;
    let fluxor_updated_at = fluxor_info.updated_at;
    let fluxor_age_in_days = ds::get_days(&format_utc_ts().to_string(), &fluxor_created_at);
    let fluxor_age_in_years = ds::get_days(&format_utc_ts().to_string(), &fluxor_created_at) as f64 / 365 as f64;
    let fluxor_age_in_years_fix_1 = math::fix(fluxor_age_in_years, 1);

    let mathlab_total_downloads = mathlab_info.total_downloads;
    let mathlab_latest = mathlab_info.latest;
    let mathlab_versions = mathlab_info.versions;
    let mathlab_health_index = mathlab_total_downloads / mathlab_versions;
    let mathlab_health_description = ds::get_health_description(mathlab_health_index);
    let mathlab_license = mathlab_info.license;
    let mathlab_created_at = mathlab_info.created_at;
    let mathlab_updated_at = mathlab_info.updated_at;
    let mathlab_age_in_days = ds::get_days(&format_utc_ts().to_string(), &mathlab_created_at);
    let mathlab_age_in_years = ds::get_days(&format_utc_ts().to_string(), &mathlab_created_at) as f64 / 365 as f64;
    let mathlab_age_in_years_fix_1 = math::fix(mathlab_age_in_years, 1);

    boxed(async move {
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
                cans_age_in_days = cans_age_in_days,
                cans_age_in_years = cans_age_in_years,
                cans_age_in_years_fix_1 = cans_age_in_years_fix_1,
                fluxor_total_downloads = fluxor_total_downloads,
                fluxor_latest = fluxor_latest,
                fluxor_versions = fluxor_versions,
                fluxor_health_index = fluxor_health_index,
                fluxor_health_description = fluxor_health_description,
                fluxor_license = fluxor_license,
                fluxor_created_at = fluxor_created_at,
                fluxor_updated_at = fluxor_updated_at,
                fluxor_age_in_days = fluxor_age_in_days,
                fluxor_age_in_years = fluxor_age_in_years,
                fluxor_age_in_years_fix_1 = fluxor_age_in_years_fix_1,
                mathlab_total_downloads = mathlab_total_downloads,
                mathlab_latest = mathlab_latest,
                mathlab_versions = mathlab_versions,
                mathlab_health_index = mathlab_health_index,
                mathlab_health_description = mathlab_health_description,
                mathlab_license = mathlab_license,
                mathlab_created_at = mathlab_created_at,
                mathlab_updated_at = mathlab_updated_at,
                mathlab_age_in_days = mathlab_age_in_days,
                mathlab_age_in_years = mathlab_age_in_years,
                mathlab_age_in_years_fix_1 = mathlab_age_in_years_fix_1
            )
        );

        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from(content))
            .unwrap())
    })
}