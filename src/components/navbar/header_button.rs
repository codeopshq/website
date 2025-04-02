use yew::prelude::*;

#[function_component(HeaderButton)]
pub fn header_button() -> Html {
    html! {
        <div data-header-buttons="" class="hidden lg:flex items-center justify-between flex-1 mx-5 pl-5 space-x-1 border-l text-gray-400 border-gray-900/10 dark:border-gray-50/10">
            <button data-action="search#open" class="h-9 w-9 flex items-center justify-center flex-shrink-0 rounded-lg hover:bg-gray-100 hover:text-gray-700 dark:hover:bg-gray-700 dark:hover:text-gray-300" title="Search" aria-label="Search">
                <svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11ZM2 9a7 7 0 1 1 12.452 4.391l3.328 3.329a.75.75 0 1 1-1.06 1.06l-3.329-3.328A7 7 0 0 1 2 9Z" clip-rule="evenodd"></path>
                </svg>
            </button>

            <div class="flex items-center">
                <a data-social-link="" class="[&amp;:nth-child(n_+_7)]:hidden h-9 w-9 flex items-center justify-center flex-shrink-0 rounded-lg hover:bg-gray-100 hover:text-gray-700 dark:hover:bg-gray-700 dark:hover:text-gray-300" href="https://youtube.com/@CodeOpsHQ" title="YouTube" aria-label="YouTube" target="_blank">
                <svg aria-hidden="true" data-prefix="fab" data-icon="youtube" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512" class="svgyoutube  ">
                  <path fill="currentColor" d="M549.655 124.083c-6.281-23.65-24.787-42.276-48.284-48.597C458.781 64 288 64 288 64S117.22 64 74.629 75.486c-23.497 6.322-42.003 24.947-48.284 48.597-11.412 42.867-11.412 132.305-11.412 132.305s0 89.438 11.412 132.305c6.281 23.65 24.787 41.5 48.284 47.821C117.22 448 288 448 288 448s170.78 0 213.371-11.486c23.497-6.321 42.003-24.171 48.284-47.821 11.412-42.867 11.412-132.305 11.412-132.305s0-89.438-11.412-132.305zm-317.51 213.508V175.185l142.739 81.205-142.739 81.201z"></path>
                </svg>
                </a>
            </div>

            <div class="flex items-center">
                // <a data-turbo-stream="true" class="text-sm font-semibold px-4 py-2 rounded-lg bg-accent text-white hover:bg-accent/90 transition-colors dark:bg-blue-400" href="/user_sessions/new">{"Sign in"}</a>
            </div>
        </div>
    }
}
