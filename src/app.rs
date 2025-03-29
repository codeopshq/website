use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
      let link_classes = "text-white hover:underline hover:bg-gray-600 px-4 py-2 rounded";
      let links = [
        ("Home", "/"),
        ("About", "/about"),
        ("Contact", "/contact"),
      ];
    html! {
        <main>
            <div class="flex flex-col h-screen">
                <nav class="h-16 px-8 py-10 shadow">
                    <div class="container flex mx-auto gap-6 items-center h-full">
                        // <h1 class="font-bold text-2xl text-white">{"CodeOps HQ"}</h1>
                        <a href="/"><img class="w-20" src="assets/img/codeops-hq-logo-icon.svg" alt="Logo" /></a>
                        <div class="flex-1"></div>
                        {for links.iter().map(|(label, href)| html! {
                            <a class={link_classes} href={*href}>{label}</a>
                        })}
                    </div>
                </nav>
                <div class="flex-1 flex py-16 px-8 items-center justify-center">
                      <img class="w-80" src="assets/img/codeops-hq-logo.svg" alt="Logo" />
                </div>

                <div class="flex-1 flex items-center justify-center">
                      <h2 class="text-4xl">{"What is CodeOps HQ?"}</h2>
                </div>

                <div class="flex-1 flex py-16 px-8 items-center gap-6 justify-center">
                    {view_card("YouTube", None, html! {
                        <>
                        <p>{"CodeOps HQ is a YouTube channel where I post videos about programming and web development."}</p>
                        <p>{"Subscribe to the channel here: "}<a class="text-blue-400" href="https://www.youtube.com/@CodeOpsHQ?sub_confirmation=1" target="_blank" rel="noopener noreferrer">{"Subscribe"}</a></p>
                        </>
                    })}
                    {view_card("GitHub", Some("yew.svg"), html! {
                        <>
                        <p>{"CodeOps HQ Hosts a collection of open source projects on GitHub."}</p>
                        <p>{"Check out the projects here: "}<a class="text-blue-400" href="https://github.com/CodeOpsHQ" target="_blank" rel="noopener noreferrer">{"GitHub"}</a></p>
                        </>
                    })}
                    {view_card("Tailwind CSS", None, html! {
                        <p>{"Tailwind CSS is a library for styling markup using a comprehensive set of utility classes, no CSS required."}</p>
                    })}
                </div>
            </div>
        </main>
    }
}

fn view_card(title: &'static str, img_url: Option<&'static str>, content: Html) -> Html {
    html! {
        <div class="w-96 h-68 rounded bg-gray-300 dark:bg-gray-800 text-white p-6">
            {for img_url.map(|url| html! {
                <img class="float-right w-12" src={url} alt="Logo" />
            })}
            <h1 class="text-4xl mb-8">{title}</h1>
            {content}
        </div>
    }
}
