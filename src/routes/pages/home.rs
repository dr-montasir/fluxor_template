use fluxor::prelude::*;

use crate::components::*;

const MAIN_HOME_CONTENT: &str = r####"<!-- Hero Block -->
            <section class="hero">
                <div class="container">
                    <!-- badge -->
                    {{BADGE}}
                    <h1 class="hero__title">
                        Rust Web. <br><span class="hero__title-accent">Simplified.</span>
                    </h1>
                    <p class="hero__description">
                        High-performance, versatile, asynchronous Rust web framework designed for data science and computing science applications.
                    </p>
                    <div class="hero__actions">
                        <a href="#get-started" class="btn btn--primary">Get Started</a>
                        <div id="get-started" class="btn btn--outline">cargo add fluxor</div>
                    </div>
                </div>
            </section>

            <!-- Code Block -->
            <section class="container" x-data="{ 
                code: 'use fluxor::prelude::*;\n\nfn hello(_req: Req, _params: Params) -> Reply {\n    boxed(async {\n       Ok(Response::builder()\n           .header(&quot;Content-Type&quot;, &quot;text/html; charset=UTF-8&quot;)\n           .body(Body::from(&quot;<h1>👋 Hello, World!</h1>&quot;)\n           .unwrap())\n    })\n}\n\n#[tokio::main]\nasync fn main() {\n    let mut app = Fluxor::new();        // Initialize the application.\n    app.route(GET, &quot;/&quot;, hello);         // Set the route (method, path, handler).\n    app.run(&quot;127.0.0.1&quot;, &quot;8080&quot;).await; // Start the HTTP server (host, port).\n}',
                copied: false,
                copy() {
                    navigator.clipboard.writeText(this.code.replaceAll('&quot;', '&quot;'));
                    this.copied = true;
                    setTimeout(() => this.copied = false, 2000);
                }
            }">
                <h2 class="sr-only">Example Code</h2>
                <div class="code-editor">
                    <div class="code-editor__header">
                        <div class="code-editor__dots">
                            <div class="code-editor__dot code-editor__dot--red"></div>
                            <div class="code-editor__dot code-editor__dot--yellow"></div>
                            <div class="code-editor__dot code-editor__dot--green"></div>
                        </div>
                        <button class="code-editor__copy" x-on:click="copy">
                            <span x-text="copied ? 'Copied!' : 'Copy Code'"></span>
                        </button>
                    </div>
                    <pre class="code-editor__content"><b><code>use fluxor::prelude::*;<br><br>fn hello(_req: Req, _params: Params) -> Reply {<br>    boxed(async {<br>       Ok(Response::builder()<br>           .header("Content-Type", "text/html; charset=UTF-8")<br>           .body(Body::from("&lt;h1&gt;👋 Hello, World!&lt;/h1&gt;"))<br>           .unwrap())<br>    })<br>}<br><br>#[tokio::main]<br>async fn main() {<br>    let mut app = Fluxor::new();        // Initialize the application.<br>    app.route(GET, "/", hello);         // Set the route (method, path, handler).<br>    app.run("127.0.0.1", "8080").await; // Start the HTTP server (host, port).<br>}</code></b></pre>
                </div>
            </section>

            <!-- Features Block -->
            <section class="features">
                <div class="container">
                    <h2 class="sr-only">Core Features</h2>
                    <div class="features__grid">
                        <div class="feature">
                            <h3 class="feature__title">/ 01.  Modular Architecture</h3>
                            <p class="feature__text">Core, Client, Data, Math, Cans, Wtime, Fluxio—versatile and well-structured modules tailored to a wide range of needs.</p>
                        </div>
                        <div class="feature">
                            <h3 class="feature__title">/ 02. Built-in API Tester</h3>
                            <p class="feature__text">Client module for quick API testing—suitable for basic testing and requires no external tools.</p>
                        </div>
                        <div class="feature">
                            <h3 class="feature__title">/ 03. Database Compatibility</h3>
                            <p class="feature__text">Data module optimized for seamless interaction with MySQL, supporting effective database management.</p>
                        </div>
                        <div class="feature">
                            <h3 class="feature__title">/ 04.   Rich Utility Functions</h3>
                            <p class="feature__text">Includes Math for computations and Wtime for time-related functionalities, enhancing high performance and productivity.</p>
                        </div>
                        <div class="feature">
                            <h3 class="feature__title">/ 05. Web Content Management </h3>
                            <p class="feature__text">Cans template engine offers elegant, lightweight, and robust web content rendering with regional and MIME type support.</p>
                        </div>
                        <div class="feature">
                            <h3 class="feature__title">/ 06. Framework Management</h3>
                            <p class="feature__text">Fluxor CLI offers rapid project scaffolding and management of Rust projects in data science and computing.</p>
                        </div>
                    </div>
                </div>
            </section>"####;

const SOURCES: &str = r##"<link rel="stylesheet" href="/css/styles.css">
    <script defer src="/js/alpine.min.js"></script>"##;

pub fn home_page(_req: Req, _params: Params) -> Reply {
    boxed(async move {
        // Await the badge function
        let badge_html = crate::components::badge("fluxor").await;

        // Generate the content with the badge
        let content = layout(
            "Fluxor", 
            "Fluxor is a versatile Rust web framework designed for data science and computing science applications.",
            "async, data-science, fluxor, framework, web",
            &do_html!(SOURCES,),
            &do_html!(MAIN_HOME_CONTENT, BADGE = badge_html)
        );

        // Return the HTTP response
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from(content))
            .unwrap())
    })
}