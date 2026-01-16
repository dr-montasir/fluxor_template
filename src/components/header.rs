use fluxor::cans::content::do_html;
use crate::components::{logo::logo, nav::{DESKTOP_NAV, MOBILE_NAV}};

const HEADER: &str = r##"<header class="header">
                <div class="container header__inner">
                    <a href="/" class="spinner-on-click header__logo">
                    {{LOGO}}
                    <span style="margin-left: 0.5rem">Fluxor</span>
                    </a>

                    <!-- Desktop Nav -->
                    {{DESKTOP_NAV}}

                    <!-- Burger Button -->
                    <button class="header__burger" x-on:click="mobileMenu = !mobileMenu" aria-label="Toggle menu">
                        <svg x-show="!mobileMenu" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            <path d="M4 6h16M4 12h16m-7 6h7"></path>
                        </svg>
                        <svg x-show="mobileMenu" x-cloak width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            <path d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                    </button>
                </div>

                <!-- Mobile Nav -->
                {{MOBILE_NAV}}
            </header>"##;

pub fn header() -> String {
    do_html!(HEADER, LOGO = logo("40", "40"), DESKTOP_NAV=DESKTOP_NAV, MOBILE_NAV=MOBILE_NAV)
}