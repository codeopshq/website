use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let link_classes = "text-gray-900 dark:text-white text-xl text-extrabold hover:text-red-400 hover:scale-110 px-1 py-2 rounded transition duration-300 ease-in-out";
    let links = [("Home", "/"), ("About", "/about"), ("Contact", "/contact")];
    html! {
        <nav class="h-16 px-8 py-10 shadow-xl">
            <div class="container flex mx-auto gap-6 items-center h-full">
                // <h1 class="font-bold text-2xl text-white">{"CodeOps HQ"}</h1>
                <a href="/"><img class="w-20" src="assets/img/codeops-hq-logo-icon.svg" alt="Logo" /></a>
                <div class="flex-1"></div>
                {for links.iter().map(|(label, href)| html! {
                <a class={link_classes} href={*href}>{label}</a>
                })}
            </div>
        </nav>
    }
}
