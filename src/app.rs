use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{footer::footer::Footer, navbar::navbar::Navbar};
use crate::pages::{home::Home, not_found::NotFound, tutorials::Tutorials};
use crate::router::Route;

#[function_component(App)]
pub fn app() -> Html {
    html! {
            <div id="viewport" class="min-h-screen flex flex-col">
                <div data-controller="slideover header-scroll">
                    <div class="fixed right-0 top-0 overflow-hidden pointer-events-none">
                      <div class="relative w-[600px] h-[600px] -translate-y-2/3 translate-x-2/3 lg:-translate-y-1/2 lg:translate-x-1/2 bg-gradient-to-bl from-accent to-transparent rounded-full blur-[100px] opacity-50"></div>
                    </div>
                    <Navbar />
                </div>
                <main class="flex-none outline-none pt-20 overflow-y-auto">
                    <div class="mx-auto max-w-7xl px-4 lg:px-8">
                        <BrowserRouter>
                            <Switch<Route> render={switch} />
                        </BrowserRouter>
                    </div>
                    <Footer />
                </main>
            </div>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Tutorials => html! { <Tutorials /> },
        Route::About => html! { <h1>{ "About" }</h1> },
        Route::Contact => html! { <h1>{ "Contact" }</h1> },
        Route::NotFound => html! { <NotFound /> },
    }
}
