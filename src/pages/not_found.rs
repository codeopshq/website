use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
        <div class="flex pt-10 items-center justify-center">
            <h1 class="text-9xl px-4 pt-20 font-extrabold">{"404"}</h1>
        </div>
        <div class="flex items-center justify-center">
            <h2 class="text-5xl px-4 py-2 font-extrabold">{"Page Not Found"}</h2>
        </div>
        <div class="flex pt-10 items-center justify-center">
            <a class="text-sm font-semibold px-4 py-2 rounded-lg bg-accent text-white hover:bg-accent/90 transition-colors dark:bg-blue-400" href="/">{"Go Home"}</a>
        </div>
        </>
    }
}
