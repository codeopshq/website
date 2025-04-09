use yew::prelude::*;

use crate::api::youtube::channel_videos::ChannelVideos;

#[function_component(Tutorials)]
pub fn tutorials() -> Html {
    html! {
    <>
        <div class="flex py-16 items-center justify-center">
            <h2 class="text-5xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-red-400 to-blue-500">
                {"Tutorials Videos"}
            </h2>
        </div>
        <ChannelVideos />
    </>
    }
}
