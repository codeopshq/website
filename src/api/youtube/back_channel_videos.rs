use crate::api::youtube::youtube_api::{
    get_channel_uploads_playlist, get_playlist_videos, PlaylistItem,
};
use anyhow::Error;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(ChannelVideos)]
pub fn channel_videos() -> Html {
    // Explicit type annotations for all states
    let videos = use_state(|| Vec::<PlaylistItem>::new());
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let page_token = use_state(|| Option::<String>::None);

    let api_key = "AIzaSyB02i2UtqzpSILfuFAvTj23fXqqjnO6hhw"; // Replace with your API key
    let channel_id = "UCbdSso-vnvKjI6E2h_hDLFA"; // Example channel

    {
        let videos = videos.clone();
        let loading = loading.clone();
        let error = error.clone();
        let page_token = page_token.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                match get_channel_uploads_playlist(api_key, channel_id).await {
                    Ok(playlist_id) => {
                        let mut all_videos = Vec::new();
                        let mut next_page_token = None;

                        loop {
                            match get_playlist_videos(
                                api_key,
                                &playlist_id,
                                next_page_token.as_deref(),
                            )
                            .await
                            {
                                Ok(response) => {
                                    all_videos.extend(response.items);
                                    next_page_token = response.next_page_token.clone();

                                    if next_page_token.is_none() {
                                        break;
                                    }
                                }
                                Err(e) => {
                                    error.set(Some(e.to_string()));
                                    break;
                                }
                            }
                        }

                        videos.set(all_videos);
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="container mx-auto p-4">
            if *loading {
                <p>{"Loading..."}</p>
            } else if let Some(err) = &*error {
                <p class="text-red-500">{ err }</p>
            } else {
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {for videos.iter().map(|video| {
                    html! {
                        <div class="w-full overflow-hidden shadow-xl bg-gray-300 hover:bg-gray-400 dark:bg-gray-700/40 dark:hover:bg-gray-700/60 y-6 hover:scale-105 rounded-xl transition-transform ease-in-out duration-500 border border-gray-700">
                            <a href={format!("https://youtu.be/{}", video.content_details.video_id)} target="_blank" class="no-underline">
                                <img
                                    class="w-full m-0 mb-1 rounded-t-xl"
                                    src={video.snippet.thumbnails.medium.url.clone()}
                                    alt={video.snippet.title.clone()}
                                />
                                <section class="p-5">
                                    <h5 class="text-lg font-bold tracking-tight text-gray-900 dark:text-white mb-3">{ video.snippet.title.clone() }</h5>
                                    <p class="text-gray-700 dark:text-gray-400 min-h-[100px]">{ video.snippet.title.clone() }</p>
                                        <span class="text-gray-700 dark:text-gray-400">{ video.snippet.published_at.clone() }</span>
                                        /* {for tutorial.tags.iter().map(|(tag, color)| {
                                            let classes = format!("tag inline-flex items-center rounded-md mx-1 text-sm font-medium px-1 py-1 {}", color);
                                            html! {
                                            <span class={classes}>
                                                { *tag }
                                            </span>
                                            }
                                        })} */
                                </section>
                            </a>
                        </div>
                    }

                })}
                </div>
            }
        </div>
    }
}
