use fluxor::cans::content::do_html;

const FOOTER: &str = r#"<footer class="footer">
            <div class="container">
                <p class="footer__text">Fluxor Framework &copy; 2025 — {{YEAR}} • Released under MIT License</p>
            </div>
        </footer>"#;

pub fn footer(year: u64) -> String {
    do_html!(FOOTER, YEAR=year)
}