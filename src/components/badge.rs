use fluxor::cans::content::do_html;
use crator::crate_data;

const BADGE: &str = r#"<div class="badge">
                    <span class="badge__label">Latest:</span>
                    <span class="lowercase">v{{latest_version}}</span>
                    <span class="badge__label badge__label--accent">Downloads:</span>
                    <span class="lowercase">{{downloads}}</span>
                    <span class="badge__label badge__label--light-muted">License:</span>
                    <span class="mobile-only uppercase">{{first_license}}</span>
                    <span class="desktop-only uppercase">{{license}}</span>
                </div>"#;

pub async fn badge(crate_name: &str) -> String {
    let crate_info = crate_data(crate_name).await;
    match crate_info {
        Ok(info) => {
            do_html!(
                BADGE, 
                latest_version = info.latest, 
                downloads = info.downloads,
                first_license = info.license.split_whitespace().next().unwrap_or(""),
                license = info.license
            )
        }
        Err(_) => "Error fetching crate info".to_string(),
    }
}