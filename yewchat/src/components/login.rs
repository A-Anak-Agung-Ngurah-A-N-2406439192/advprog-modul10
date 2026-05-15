use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();

        Callback::from(move |_| {
            *user.username.borrow_mut() = (*username).clone();
        })
    };

    html! {
        <div class="bg-gradient-to-br from-gray-900 via-slate-800 to-violet-900 flex w-screen h-screen">
            <div class="container mx-auto flex flex-col justify-center items-center text-white">
                <div class="text-center mb-8">
                    <div class="text-5xl mb-3">{"🦀💬"}</div>
                    <h1 class="text-4xl font-bold">{"Abhivadya's YewChat"}</h1>
                    <p class="text-sm text-gray-300 mt-2">
                        {"A creative async webchat powered by Rust, Yew, and WebSocket"}
                    </p>
                </div>

                <form class="m-4 flex shadow-2xl">
                    <input
                        {oninput}
                        class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white"
                        placeholder="Enter your username"
                    />

                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="px-8 rounded-r-lg bg-violet-600 hover:bg-violet-700 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r"
                        >
                            {"Start Chatting 🚀"}
                        </button>
                    </Link<Route>>
                </form>

                <p class="text-xs text-gray-400 mt-4">
                    {"Creativity makes technology feel more human."}
                </p>
            </div>
        </div>
    }
}