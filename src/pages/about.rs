use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <div class="flex py-16 items-center justify-center">
                <h2 class="text-5xl bg-red-400 text-gray-100 dark:text-gray-900 px-4 py-2 font-extrabold">{"About"}</h2>
            </div>
            <div class="flex items-center justify-center">
              <img src="assets/img/codeops-hq-logo.svg" class="w-20 h-20 mr-4"/>
              <div>
                <strong>{"CodeOps HQ"}</strong>
                <span>{"Programmming, DevOps and Linux"}</span>
              </div>
            </div>
        </>
    }
}
