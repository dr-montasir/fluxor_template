pub const DESKTOP_NAV: &str = r##"<nav class="header__nav">
                        <a 
                            href="/" 
                            class="spinner-on-click header__link"
                            x-data="{ show: false }"
                            x-init="show = (window.location.pathname !== '/')"
                            x-show="show"
                        >
                            Home
                        </a>
                        <a href="/analytics" class="spinner-on-click header__link">Analytics</a>
                        <a href="https://docs.rs/fluxor/latest/fluxor" class="header__link" target="_blank">Docs</a>
                        <a href="https://github.com/dr-montasir/fluxor" class="header__link" target="_blank">GitHub</a>
                        <a 
                            href="#get-started" 
                            class="header__link header__link--button"
                            x-data="{ show: false }" 
                            x-init="show = (window.location.pathname === '/')"
                            x-show="show"
                        >
                            Get Started
                        </a>
                    </nav>"##;

pub const MOBILE_NAV: &str = r##"<div class="header__mobile-nav" x-show="mobileMenu" x-cloak x-transition x-on:click.away="mobileMenu = false">
                    <a href="/" class="spinner-on-click header__link">Home</a>
                    <a href="/analytics" class="spinner-on-click header__link">Analytics</a>
                    <a href="https://docs.rs/fluxor/latest/fluxor" class="header__link" target="_blank">Docs</a>
                    <a href="https://github.com/dr-montasir/fluxor" class="header__link" target="_blank">GitHub</a>
                    <a 
                        href="#get-started" 
                        class="header__link header__link--button" 
                        style="text-align: center;"
                        x-data="{ show: false }" 
                        x-init="show = (window.location.pathname === '/')"
                        x-show="show"
                    >
                        Get Started
                    </a>
                </div>"##;