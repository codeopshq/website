use yew::prelude::*;

use crate::components::footer::icon::Icon;

#[function_component(Footer)]
pub fn footer() -> Html {
    let icons = [
        ("youtube", "https://youtube.com/@CodeOpsHQ"),
        ("twitter", "https://twitter.com/CodeOpsHQ"),
        ("github", "https://github.com/CodeOpsHQ"),
        ("twitch", "https://twitch.tv/codeopshq"),
        ("discord", "https://discord.gg/pfWCjYA8"),
        ("facebook", "https://www.facebook.com/codeopshq"),
        ("tiktok", "https://www.tiktok.com/@codeopshq"),
        ("instagram", "https://www.instagram.com/codeopshq/"),
        ("linkedin", "https://www.linkedin.com/company/codeopshq/"),
    ];
    html! {
            <footer class="text-center my-6 p-8 text-gray3">
                <div class="mx-auto w-24 h-1 my-12 bg-gradient-to-r from-gray5 to-gray4 rounded-full"></div>
                <div class="pt-10">{ "Find an issue with this page?"  }<a class="text-blue-500" href="https://github.com/codeopshq/website/tree/main">{ "Fix it on GitHub" }</a></div>

                <div class="flex justify-center items-center my-2">

                    {for icons.iter().map(|(svg, url)| html! {
                        <Icon url={*url} svg={*svg} />
                    })}

                </div>

                /* <h6>{ "Helpful Links" }</h6>

                <div class="py-3">
                    <a href="/courses">{ "Courses" }</a>{" | "}
                    <a href="/lessons">{ "Labs" }</a>{" | "}
                    <a href="/snippets">{ "Snippets" }</a>{" | "}
                    <a href="/tags">{ "Tags" }</a>{" | "}
                    <a href="/contributors">{ "Contrib" }</a>{" | "}
                    <a href="/privacy">{ "Privacy" }</a>{" | "}
                    <a href="/terms">{ "Terms" }</a>
                </div> */


                <div class="text-xs">
                    { "Copyright © " } {chrono::Local::now().format("%Y").to_string()} { " CodeOps HQ" } <br />
                </div>

            </footer>
    }
}
