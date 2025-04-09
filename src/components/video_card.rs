use chrono::{DateTime, Utc};
use yew::prelude::*;

use crate::api::youtube::youtube_api::PlaylistItem;

#[derive(Properties, PartialEq, Clone)]
pub struct VideoCardProps {
    pub video: PlaylistItem,
    pub index: usize,
}

#[function_component(VideoCard)]
pub fn video_card(VideoCardProps { video, index }: &VideoCardProps) -> Html {
    let formatted_date = format_published_date(&video.snippet.published_at);

    // Fallback for missing thumbnails
    let thumbnail_url = video.snippet.thumbnails.medium.url.clone();

    html! {
        <div key={*index} class="w-full overflow-hidden bg-gray-300 hover:bg-gray-400 dark:bg-gray-700/40 dark:hover:bg-gray-700/60 rounded-xl backdrop-blur-sm ring-1 ring-white/10 transition-all duration-300 group">
            <a href={format!("https://youtu.be/{}", video.content_details.video_id)} target="_blank" aria-label={format!("Watch video: {}", video.snippet.title.clone())} class="no-underline block">
                <img
                    class="w-full aspect-video object-cover rounded-t-xl group-hover:scale-105 transition-transform ease-in-out duration-500"
                    src={thumbnail_url}
                    alt={format!("Thumbnail for video: {}", video.snippet.title.clone())}
                />
                <section class="p-5 space-y-2">
                    <h5 class="text-lg font-bold tracking-tight text-gray-900 dark:text-white line-clamp-2">{ video.snippet.title.clone() }</h5>
                    <p class="text-sm text-gray-700 dark:text-gray-400 line-clamp-3">{ video.snippet.description.clone() }</p>
                    <div class="flex items-center space-x-2 pt-2">
                        <span class="text-xs text-gray-500 dark:text-gray-400">{ formatted_date }</span>
                        /* {
                            for video.snippet.tags.iter().map(|tag| {

                            })
                        }
                        <span class="inline-flex items-center rounded-full bg-green-500 dark:bg-green-600 text-white px-2 py-1 text-xs font-medium">
                            { "#Rust" } // Replace with dynamic tags if available
                        </span> */
                    </div>
                </section>
            </a>
        </div>
    }
}

fn format_published_date(date_string: &str) -> String {
    let datetime = DateTime::parse_from_rfc3339(date_string).unwrap_or_else(|_| Utc::now().into()); // Fallback to current time if parsing fails
    datetime.format("%d %b %Y").to_string()
}
