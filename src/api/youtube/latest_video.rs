use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::youtube::youtube_api::fetch_latest_video;

use super::youtube_api::PlaylistItem;

#[function_component(LatestVideo)]
pub fn latest_video() -> Html {
    let latest_video = use_state(|| None::<PlaylistItem>);

    {
        let latest_video = latest_video.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match fetch_latest_video().await {
                    Ok(video) => latest_video.set(Some(video)),
                    Err(err) => {
                        gloo_console::error!(format!("Error fetching latest video: {}", err))
                    }
                }
            })
        });
    }

    html! {
    <>
        <div class="rounded-2xl bg-gray-300 dark:bg-gray-700/40 backdrop-blur-sm ring-1 ring-white/10 h-full transition-all hover:bg-gray-300/50 dark:hover:bg-gray-700/60 duration-300">
        { if let Some(video) = &*latest_video {
            html! {
                <div class="aspect-video rounded-t-2xl overflow-hidden">
                    <iframe
                        class="w-full h-full"
                        src={format!("https://www.youtube.com/embed/{}", video.content_details.video_id)}
                        title={video.snippet.title.clone()}
                        frameborder="0"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                        referrerpolicy="strict-origin-when-cross-origin"
                        allowfullscreen=true>
                    </iframe>
                </div>
            }
        } else {
            html! {
                <div class="animate-pulse aspect-video rounded-t-2xl overflow-hidden">
                    <div class="w-full h-full bg-gray-200 dark:bg-black flex items-center justify-center">
                        <h1 class="text-3xl font-bold text-gray-600 dark:text-gray-400">{"Loading..."}</h1>
                    </div>
                </div>
            }
        }}
        <div class="p-6">
            <h3 class="text-sm font-semibold text-gray-600 dark:text-gray-400">{ "Welcome to CodeOps HQ" }</h3>
            <p class="mt-2 text-xl font-medium text-black dark:text-white">{ "Love What You Build" }</p>
        </div>
        </div>
    </>
    }
}
