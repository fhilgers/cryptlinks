use leptos::leptos_dom::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use cl_core::{create_encoded_payload, decrypt_encoded_payload};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    //<Route path="/" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/scanner" view=|cx| view! { cx, <Scanner/> }/>
                    <Route path="/:payload" view=|cx| view! { cx, <Decryptor/ >}/>
                    /*
                    <Route path="/encrypt" view=|cx| view! { cx, <HomePage/> }>
                        <Route path="chacha20poly1305" view=|cx| view! { cx, <Chacha20Poly1305Encrypt/> }/>
                    </Route>
                    <Route path="/decrypt" view=|cx| view! { cx, <HomePage/> }>
                        <Route path="chacha20poly1305" view=|cx| view! { cx, <HomePage/> }/>
                    </Route>
                    <Route path="/chacha20poly1305/:payload" view=|cx| view! { cx, <HomePage/> }/>
                    */
                </Routes>
            </main>
        </Router>

    }
}

#[component]
fn Scanner(cx: Scope) -> impl IntoView {
    let navigate = use_navigate(cx);

    window_event_listener("scan", move |_| {
        let storage = window().local_storage().unwrap().unwrap();
        let link = storage.get("payload").unwrap().unwrap();
        let u = url::Url::parse(&link).unwrap();
        let payload = u.path();
        navigate(payload, NavigateOptions::default()).unwrap();
    });

    view! {cx,
        <script src="https://unpkg.com/html5-qrcode" type="text/javascript"></script>
        <script>
        "
            const script = document.createElement('script');
            script.src = 'https://unpkg.com/html5-qrcode';
            document.body.appendChild(script);
            var lastResult, countResults = 0;
            function onScanSuccess(decodedText, decodedResult) { 
                if (decodedText !== lastResult) { 
                    ++countResults; 
                    lastResult = decodedText; 
                    const event = document.createEvent('Event');
                    event.initEvent('scan', true, true);
                    localStorage.setItem('payload', decodedText);
                    window.dispatchEvent(event);
                } 
            }
            script.onload = () => { 
                var html5QrcodeScanner = new Html5QrcodeScanner('qr-reader', { fps: 10, qrbox: 250 }); 
                html5QrcodeScanner.render(onScanSuccess); 
            };
        "
        </script>
        <div class="container max-w-screen max-h-screen mx-auto py-24 px-6">
            <div class="font-sans">
                <div class="mx-auto px-6">
                    <div class="relative flex flex-wrap">
                        <div class="w-full h-screen relative">
                            <div class="mt-6">
                                <div class="py-2" >
                                    <div id="qr-reader" class="relative"></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Decryptor(cx: Scope) -> impl IntoView {
    let (password, set_password) = create_signal(cx, "".to_string());
    let (text, set_text) = create_signal(cx, None);

    let params = use_params_map(cx);

    let payload = move || {
        console_log("getting payload");
        params.with(|params| params.get("payload").cloned())
    };

    let decrypt = move |_| {
        console_log("trying to decrypt");
        match payload() {
            Some(p) => {
                let decrypted = decrypt_encoded_payload(&password.get(), &p);
                set_text(Some(decrypted));
            }
            None => {
                set_text(Some("Unable to decrypt".to_string()));
            }
        }
    };

    view! {cx,
        <div class="container max-w-full mx-auto py-24 px-6">
            <div class="font-sans">
                <div class="max-w-sm mx-auto px-6">
                    <div class="relative flex flex-wrap">
                        <div class="w-full relative">
                            <div class="mt-6">
                                <div class="py-2" >
                                    <span class="px-1 text-sm text-gray-600">"Password"</span>
                                    <div class="relative">
                                        <input placeholder="" type="password" on:input=move |ev| set_password(event_target_value(&ev)) class="text-md block px-3 py-2 rounded-lg w-full
                                        bg-white border-2 border-gray-300 placeholder-gray-600 shadow-md
                                        focus:placeholder-gray-500
                                        focus:bg-white 
                                        focus:border-gray-600  
                                        focus:outline-none"/>
                                    </div>
                                    <button on:click=decrypt class="mt-3 text-lg font-semibold
                                    bg-gray-800 w-full text-white rounded-lg
                                    px-6 py-3 block shadow-xl hover:text-white hover:bg-black">
                                    "Decrypt"
                                    </button>
                                </div>
                                {move || match text.get() {
                                    Some(t) => view! {cx, <p>{t}</p>},
                                    None => view! { cx, <p>"Waiting"</p>}
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Chacha20Poly1305Encrypt(cx: Scope) -> impl IntoView {
    let (password, set_password) = create_signal(cx, "".to_string());
    let (text, set_text) = create_signal(cx, "".to_string());

    let (encrypted_text, set_encrypted_text) = create_signal(cx, "".to_string());
    let encrypt = move |_| {
        let payload = create_encoded_payload(&password.get(), &text.get());
        set_encrypted_text.set(payload);
    };
    view! {cx,
            <div class="flex flex-col w-full m-8 p-8 items-center">

            <label class="relative block">
            <span class="sr-only">"Search"</span>
            <span class="absolute inset-y-0 left-0 flex items-center pl-2">
              <svg class="h-5 w-5 fill-slate-300" viewBox="0 0 20 20">"<!-- ... -->"</svg>
            </span>
            <input class="placeholder:italic placeholder:text-slate-400 block bg-white w-full border border-slate-300 rounded-md py-2 pl-9 pr-3 shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" placeholder="Search for anything..." type="text" name="search"/>

            <label for="message" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Your message"</label>
    <textarea id="message" rows="4" class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Write your thoughts here..."></textarea>
          </label>
                <textarea on:input=move |ev| set_text(event_target_value(&ev))></textarea>
                <input type="password" on:input=move |ev| set_password(event_target_value(&ev))/>
                <button on:click=encrypt>"Encrypt"</button>
                <p>{encrypted_text}</p>
            </div>
        }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="h-screen w-screen flex bg-gray-200">
            <aside class="flex flex-col items-center bg-white text-gray-700 shadow w-64 h-full px-5 py-8">
            <ul>
                <li class="hover:bg-gray-100">
                <a href="/encrypt/chacha20poly1305">"Chacha20Poly1305"</a>
                </li>
            </ul>
            </aside>
            <Outlet/>
        </div>
    }
}
