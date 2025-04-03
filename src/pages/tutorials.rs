use yew::prelude::*;

//use crate::api::tutorials::get_tutorials;
use crate::api::youtube::channel_videos::ChannelVideos;

#[function_component(Tutorials)]
pub fn tutorials() -> Html {
    //let tutorials = get_tutorials();

    html! {
    <>
        <div class="flex py-16 items-center justify-center">
            <h2 class="text-5xl bg-orange-400 text-gray-100 dark:text-gray-900 px-4 py-2 font-extrabold">{"Tutorials Videos"}</h2>
        </div>
        <ChannelVideos />
    </>
    }
}
