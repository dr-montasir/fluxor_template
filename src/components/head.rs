use fluxor::cans::content::do_html;

const HEAD: &str = r#"<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta name="description" content="{{description}}" />
    <meta name="keywords" content="{{keywords}}" />
    <title>{{page_title}}</title>
    <link rel="manifest" href="/manifest.json" />
    <link rel="icon" href="/images/logo.svg" type="image/svg+xml" />
    <link href="https://fonts.googleapis.com/css2?family=Inter&display=swap" rel="stylesheet">
    {{sources}}
</head>"#;

pub fn head(title: &str, description: &str, keywords: &str, sources: &str) -> String {
    do_html!(
        HEAD,
        description=description,
        keywords=keywords,
        page_title=title,
        sources=sources
    )
}