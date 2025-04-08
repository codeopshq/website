use yew::prelude::*;

use crate::api::youtube::youtube_api::PlaylistItem;
use crate::components::video_card::VideoCard;

#[derive(Properties, PartialEq, Clone)]
pub struct VideoGridProps {
    pub videos: Vec<PlaylistItem>,
}

#[function_component(VideoGrid)]
pub fn video_grid(VideoGridProps { videos }: &VideoGridProps) -> Html {
    html! {
        <>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            { for videos.iter().enumerate().map(|(index, video)| {
                html! {
                    <VideoCard video={video.clone()} index={index} />
                }
            }) }
        </div>
        </>
    }
}
