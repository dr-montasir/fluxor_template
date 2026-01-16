use fluxor::cans::content::do_html;
use fluxor::wtime;

use crate::components::*;

pub const LAYOUT_TEMPLATE: &str = r###"<!DOCTYPE html>
<html lang="en" x-data="{ isLoading: false }" 
    x-init="
        isLoading = false;
        // Add event listeners to all <a> elements with the class 'spinner-on-click'
        document.querySelectorAll('a.spinner-on-click').forEach(link => {
            link.addEventListener('click', () => {
                isLoading = true; // Show spinner on click
            });
        });

        // When page loads, hide spinner
        document.addEventListener('DOMContentLoaded', () => {
            isLoading = false;
        });
    "
>
    <!-- head -->
    {{HEAD}}
    <body>
        <!-- Loading overlay -->
        <div 
            x-show="isLoading" 
            class="loadingscreen__overlay" 
            x-transition
        >
            <div class="loadingscreen__spinner">
                {{LOGO}}
            </div>
            
        </div>

        <!-- main container -->
        <div x-data="{ mobileMenu: false }">
            <!-- header -->
            {{HEADER}}

            <!-- main content -->
            {{MAIN_CONTENT}}

            <!-- footer -->
            {{FOOTER}}
        </div>

        <!-- scripts -->
        <!-- service worker register -->
        {{SW_REGISTER_SCRIPT}}
    </body>
</html>"###;

pub fn layout(title: &str, description: &str, keywords: &str, sources: &str, main_content: &str) -> String {
    let year = wtime::local::get_local_year();

    do_html!(
        LAYOUT_TEMPLATE,
        HEAD = head(title, description, keywords, sources),
        LOGO = logo::logo("96", "96"),
        HEADER = header(),
        MAIN_CONTENT=main_content,
        FOOTER=footer(year),
        SW_REGISTER_SCRIPT = SW_REGISTER
    )
}