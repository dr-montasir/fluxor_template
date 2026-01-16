use fluxor::prelude::*;

mod api;
mod pages;

use pages::{home_page, analytics_page};
use api::hello_world;

pub fn setup_routes(app: &mut Fluxor) {
    // api
    app.route(POST, "/api/greeting/hello", hello_world);  // Set the hello_world route.
    
    // pages
    app.route(GET, "/", home_page);                 // Set the home route.
    app.route(GET, "/analytics", analytics_page);   // Set the analytics route.

    // client
    app.route(GET, "/http-client", serve_http_client);
}