use fluxor::prelude::*;

// routes
use fluxor_template::{not_found_page, routes::setup_routes};
       
#[tokio::main]
async fn main() {
    let mut app = Fluxor::new();			     // Initialize the application.
    let static_dir = "src/assets".to_string();   // Retrieve the static directory "assets"

    app.set_dir(static_dir);            // Set directory for static files

    setup_routes(&mut app);             // Setup HTTP routes.

    // Set custom 404 handler
    app.set_custom_404(|content_type| {
        match content_type {
            "application/json" => do_json!(r#"{"error": {"code": 404, "message": "Not Found."}}"#,),
            "text/html" => not_found_page(),
            _ => do_text("404 Resource Not Found."),
        }
    });

    app.run("0.0.0.0", "10000").await;		// Start the HTTP server (host, port).
}