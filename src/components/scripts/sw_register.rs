pub const SW_REGISTER: &str = r#"<style>
            #install-btn {
                position: fixed;
                bottom: 0px;
                left: 50%;
                transform: translateX(-50%);
                padding: 1rem 2rem;
                font-size: 1.2rem;
                font-weight: bold;
                background-color: #d9740b;
                color: #fff;
                border: none;
                border-radius: 0px;
                border-radius: 15px 15px 0 0;
                width: 50%;
                cursor: pointer;
                z-index: 9999;
                box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            }
            #install-btn:hover {
                background-color: #ea580c;
            }
            @media (max-width: 768px) { /* or adjust the breakpoint as needed */
                #install-btn {
                    width: 70%;
                }
            }
        </style>
        <script>
            if ('serviceWorker' in navigator) {
                navigator.serviceWorker.register('/service-worker.js')
                    .then(function(registration) {
                        console.log('Service Worker registered with scope:', registration.scope);
                    })
                    .catch(function(error) {
                    console.log('Service Worker registration failed:', error);
                    });
            }
        </script>

        <!-- Install button -->
        <button id="install-btn" style="display: none;">Install Fluxor PWA</button>

        <!-- Install prompt handling -->
        <script>
        let deferredPrompt;

        window.addEventListener('beforeinstallprompt', (e) => {
            e.preventDefault();
            deferredPrompt = e;
            document.getElementById('install-btn').style.display = 'block';
        });

        document.getElementById('install-btn').addEventListener('click', () => {
            if (deferredPrompt) {
                deferredPrompt.prompt();
                deferredPrompt.userChoice.then((choiceResult) => {
                    if (choiceResult.outcome === 'accepted') {
                        console.log('User accepted the install prompt');
                    } else {
                        console.log('User dismissed the install prompt');
                    }
                    deferredPrompt = null;
                    document.getElementById('install-btn').style.display = 'none';
                });
            }
        });
        </script>"#;