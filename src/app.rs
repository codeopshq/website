use yew::prelude::*;

use crate::components::{footer::footer::Footer, navbar::Navbar};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <container class="flex flex-col h-screen">
                <Navbar />
                <div class="flex py-16 items-center justify-center">
                      <h2 class="text-5xl bg-green-400 px-4 py-2 font-extrabold text-gray-900">{"Our Latest Video"}</h2>
                </div>
                <div class="flex pb-16 px-8 items-center justify-center">
                    // <img class="w-80 mr-9" src="assets/img/codeops-hq-logo.svg" alt="Logo" />
                    <iframe width="560" height="315" src="https://www.youtube.com/embed/F2X9jjCT17E?si=3rQt4qJfLFnfQT7e" allow=""></iframe>
                </div>

                <div class="flex items-center justify-center">
                      <h2 class="text-4xl">{"What is CodeOps HQ?"}</h2>
                </div>

                <div class="flex py-16 px-8 items-center gap-6 justify-center">
                    {view_card("YouTube", Some("assets/img/youtube-light-logo.svg"), html! {
                        <>
                        <p>{"CodeOps HQ is a YouTube channel where I post videos about linux, devops, and programming."}</p>
                        <p>{"Subscribe to the channel here: "}<a class="text-red-400" href="https://www.youtube.com/@CodeOpsHQ?sub_confirmation=1" target="_blank" rel="noopener noreferrer">{"Subscribe"}</a></p>
                        </>
                    })}
                    {view_card("GitHub", Some("assets/img/github-logo-white.png"), html! {
                        <>
                        <p>{"CodeOps HQ Hosts a collection of open source projects on GitHub that I maintain and contribute."}</p>
                        <p>{"Check out the projects here: "}<a class="text-red-400" href="https://github.com/CodeOpsHQ" target="_blank" rel="noopener noreferrer">{"GitHub"}</a></p>
                        </>
                    })}
                    {view_card("Tailwind CSS", None, html! {
                        <p>{"Tailwind CSS is a library for styling markup using a comprehensive set of utility classes, no CSS required."}</p>
                    })}
                </div>
                <Footer />
            </container>
        </main>
    }
}

fn view_card(title: &'static str, img_url: Option<&'static str>, content: Html) -> Html {
    html! {
        <div class="w-96 rounded bg-gray-300 dark:bg-gray-800 p-6 shadow-lg">
            {for img_url.map(|url| html! {
                <img class="float-right w-20 invert dark:invert-0" src={url} alt="Logo" />
            })}
            <h1 class="text-4xl mb-8">{title}</h1>
            {content}
        </div>
    }
}
