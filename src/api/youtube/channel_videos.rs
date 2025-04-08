use crate::api::youtube::youtube_api::{
    get_channel_uploads_playlist, get_playlist_videos, PlaylistItem,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::{
    load_more_section::LoadMoreSection, skeleton_grid::SkeletonGrid, video_grid::VideoGrid,
};

#[function_component(ChannelVideos)]
pub fn channel_videos() -> Html {
    let videos = use_state(Vec::<PlaylistItem>::new);
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let page_token = use_state(|| Option::<String>::None);
    let playlist_id = use_state(|| Option::<String>::None);

    let api_key = env!("YOUTUBE_API_KEY");
    let channel_id = env!("YOUTUBE_CHANNEL_ID");

    {
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
                <SkeletonGrid />
            } else if let Some(err) = &*error {
                <p class="text-red-500">{ err }</p>
            } else {
                <VideoGrid
                    videos={(*videos).clone()}
                />
                <LoadMoreSection
                    loading={*loading}
                    page_token_exists={page_token.is_some()}
                    on_load_more={on_load_more.clone()}
                 />
            }
        </div>
    }
}
