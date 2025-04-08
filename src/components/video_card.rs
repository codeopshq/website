use yew::prelude::*;

use crate::api::youtube::youtube_api::PlaylistItem;

#[derive(Properties, PartialEq, Clone)]
pub struct VideoCardProps {
    pub video: PlaylistItem,
    pub index: usize,
}

#[function_component(VideoCard)]
pub fn video_card(VideoCardProps { video, index }: &VideoCardProps) -> Html {
    html! {
        <div key={*index} class="w-full overflow-hidden shadow-xl bg-gray-300 hover:bg-gray-400 dark:bg-gray-700/40 dark:hover:bg-gray-700/60 y-6 rounded-xl border border-gray-700">
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
}
