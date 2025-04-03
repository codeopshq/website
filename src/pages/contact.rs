use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <>
        <div class="flex py-16 items-center justify-center">
            <h2 class="text-5xl bg-sky-400 text-gray-100 dark:text-gray-900 px-4 py-2 font-extrabold">{"Contact"}</h2>
        </div>
        </>
    }
}
