use yew::prelude::*;

#[function_component(SkeletonGrid)]
pub fn skeleton_grid() -> Html {
    html! {
        <div class="flex animate-pulse space-x-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {
            for (0..6).map(|i| html! {
            <div key={i} class="rounded-xl border border-gray-300 dark:border-gray-700 overflow-hidden shadow bg-gray-200 dark:bg-gray-800/40 p-4 space-y-3">
                <div class="h-32 sm:h-40 bg-gray-300 dark:bg-gray-700 rounded"></div>
                <div class="space-y-2">
                    <div class="h-4 bg-gray-300 dark:bg-gray-700 rounded"></div>
                    <div class="h-3 bg-gray-300 dark:bg-gray-700 rounded w-5/6"></div>
                    <div class="h-3 bg-gray-300 dark:bg-gray-700 rounded w-3/4"></div>
                </div>
            </div>
            })
            }
        </div>
    }
}
