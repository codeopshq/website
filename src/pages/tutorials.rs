use yew::prelude::*;

use crate::api::tutorials::get_tutorials;

#[function_component(Tutorials)]
pub fn tutorials() -> Html {
    let tutorials = get_tutorials();

    html! {
    <>
        <div class="flex py-16 items-center justify-center">
            <h2 class="text-5xl px-4 py-2 font-extrabold">{"Tutorials Videos"}</h2>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {for tutorials.iter().map(|tutorial| {
                html! {
                    <div class="w-full overflow-hidden shadow-xl bg-gray-300 hover:bg-gray-400 dark:bg-gray-700/40 dark:hover:bg-gray-700/60 y-6 hover:scale-105 rounded-xl transition-transform ease-in-out duration-500 border border-gray-700">
                        <a href={tutorial.video_url} class="no-underline">
                            <img class="w-full m-0 mb-1 rounded-t-xl" src={tutorial.image_url} alt={ tutorial.title } />
                            <section class="p-5">
                                <h5 class="text-lg font-bold tracking-tight text-gray-900 dark:text-white mb-3">{ tutorial.title }</h5>
                                <p class="text-gray-700 dark:text-gray-400 min-h-[100px]">{ tutorial.description }</p>
                                    {for tutorial.tags.iter().map(|(tag, color)| {
                                        let classes = format!("tag inline-flex items-center rounded-md mx-1 text-sm font-medium px-1 py-1 {}", color);
                                        html! {
                                        <span class={classes}>
                                            { *tag }
                                        </span>
                                        }
                                    })}
                            </section>
                        </a>
                    </div>
                }

            })}
        </div>
    </>
    }
}
