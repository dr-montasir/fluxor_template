use fluxor::cans::content::do_html;
use crate::components::*;

const NOT_FOUND_HTML: &str = r##"<div class="error404">
                <h1 class="error404__title">
                    <span class="error404__number">404</span>
                </h1>
                <div class="error404__content">
                    <p class="error404__message">Page Not Found</p>
                    <p id="datetime" class="error404__datetime"></p>
                    <div class="hero__actions">
                        <a href="/" class="btn btn--primary">Go Back Home</a>
                    </div>
                </div>
            </div>
            <script>
                function showDateTime() {
                    const now = new Date();
                    const optionsDate = { year: 'numeric', month: 'numeric', day: 'numeric' };
                    const optionsTime = { hour: '2-digit', minute: '2-digit', second: '2-digit' };

                    const dateString = now.toLocaleDateString(undefined, optionsDate);
                    const timeString = now.toLocaleTimeString(undefined, optionsTime);

                    document.getElementById('datetime').innerHTML = dateString + '<br>' + timeString;
                }

                showDateTime();
                setInterval(showDateTime, 1000);
            </script>"##;


pub fn not_found_page () -> String {
    return layout("404 Not Found", 
        "Fluxor is a versatile Rust web framework designed for data science and computing science applications.", 
        "async, data-science, fluxor, framework, web", 
        &do_html!(r##"<link rel="stylesheet" href="/css/styles.css">
        <script defer src="/js/alpine.min.js"></script>"##,),
        NOT_FOUND_HTML
    );
}