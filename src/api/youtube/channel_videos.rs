use crate::api::youtube::youtube_api::{
    get_channel_uploads_playlist, get_playlist_videos, PlaylistItem,
};
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(ChannelVideos)]
pub fn channel_videos() -> Html {
    let videos = use_state(Vec::<PlaylistItem>::new);
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let page_token = use_state(|| Option::<String>::None);
    let playlist_id = use_state(|| Option::<String>::None);

    let api_key = "AIzaSyB02i2UtqzpSILfuFAvTj23fXqqjnO6hhw";
    let channel_id = "UCbdSso-vnvKjI6E2h_hDLFA";

    {
        //log!("initial load", api_key, channel_id);
        let videos = videos.clone();
        let loading = loading.clone();
        let error = error.clone();
        let page_token = page_token.clone();
        let playlist_id = playlist_id.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                if *loading {
                    return;
                }
                loading.set(true);

                match get_channel_uploads_playlist(api_key, channel_id).await {
                    Ok(pid) => {
                        playlist_id.set(Some(pid.clone()));

                        match get_playlist_videos(api_key, &pid, None).await {
                            Ok(response) => {
                                videos.set(response.items);
                                page_token.set(response.next_page_token);
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
                loading.set(false);
            });
            log!("cleanup");
            || ()
        });
    }

    let on_load_more = {
        let videos = videos.clone();
        let loading = loading.clone();
        let error = error.clone();
        let page_token = page_token.clone();
        let playlist_id = playlist_id.clone();
        Callback::from(move |_| {
            let videos = videos.clone();
            let loading = loading.clone();
            let error = error.clone();
            let page_token = page_token.clone();
            let playlist_id = playlist_id.clone();
            spawn_local(async move {
                if *loading {
                    return;
                }
                loading.set(true);

                if let (Some(pid), Some(token)) = (&*playlist_id, &*page_token) {
                    match get_playlist_videos(api_key, pid, Some(token)).await {
                        Ok(response) => {
                            let mut current_videos = (*videos).clone();
                            current_videos.extend(response.items);
                            videos.set(current_videos);
                            page_token.set(response.next_page_token);
                        }
                        Err(e) => error.set(Some(e.to_string())),
                    }
                }
                loading.set(false);
            });
        })
    };

    html! {
        <div class="container mx-auto p-4">
            if *loading {
                <p>{"Loading..."}</p>
            } else if let Some(err) = &*error {
                <p class="text-red-500">{ err }</p>
            } else {
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    { for videos.iter().map(|video| {
                        html! {
                            <div class="w-full overflow-hidden shadow-xl bg-gray-300 hover:bg-gray-400 dark:bg-gray-700/40 dark:hover:bg-gray-700/60 y-6 rounded-xl border border-gray-700">
                                <a href={format!("https://youtu.be/{}", video.content_details.video_id)} target="_blank" class="no-underline">
                                    <img
                                        class="w-full m-0 mb-1 rounded-t-xl hover:scale-105 transition-transform ease-in-out duration-500"
                                        src={video.snippet.thumbnails.medium.url.clone()}
                                        alt={video.snippet.title.clone()}
                                    />
                                    <section class="p-5">
                                        <h5 class="text-lg font-bold tracking-tight text-gray-900 dark:text-white mb-3">{ video.snippet.title.clone() }</h5>
                                        <p class="text-gray-700 dark:text-gray-400 min-h-[100px]">{ video.snippet.title.clone() }</p>
                                        <span class="text-gray-700 dark:text-gray-400">{ video.snippet.published_at.clone() }</span>
                                        <span class="tag inline-flex bg-green-500 items-center rounded-md mx-1 text-sm font-medium px-1 py-1">
                                            {"#Rust"}
                                        </span>
                                    </section>
                                </a>
                            </div>
                        }
                    }) }
                </div>
                    <button
                        onclick={on_load_more}
                        disabled={*loading}
                        class="mt-6 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                    >
                        { "Load More" }
                    </button>
            }
        </div>
    }
}
