use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div class="flex py-16 items-center justify-center">
                  <h2 class="text-5xl bg-green-400 px-4 py-2 font-extrabold text-gray-100 dark:text-gray-900">{"Our Latest Video"}</h2>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                <div class="col-span-1 md:col-span-2 lg:col-span-2 row-span-2">
                    <div class="rounded-2xl bg-gray-300 dark:bg-gray-700/40 backdrop-blur-sm ring-1 ring-white/10 h-full transition-all hover:bg-gray-300/50 dark:hover:bg-gray-700/60 duration-300">
                        <div class="aspect-video rounded-t-2xl overflow-hidden">
                            <iframe class="w-full h-full" src="https://www.youtube.com/embed/F2X9jjCT17E?si=qYKSexnLaNuss6ZZ" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen=true></iframe>
                        </div>
                        <div class="p-6">
                            <h3 class="text-sm font-semibold text-gray-600 dark:text-gray-400">{ "Welcome to CodeOps HQ" }</h3>
                            <p class="mt-2 text-xl font-medium text-black dark:text-white">{ "Love What You Build" }</p>
                        </div>
                    </div>
                </div>

                <div class="col-span-1">
                    <div class="rounded-2xl bg-gray-200 dark:bg-gray-600/30 backdrop-blur-sm ring-1 ring-white/10 p-6 transition-all hover:bg-gray-300 dark:hover:bg-gray-600/50 duration-300">
                        <h3 class="text-sm font-semibold text-gray-400">{ "Community" }</h3>
                        <p class="mt-2 text-2xl font-medium text-white">{ "100,000+" }</p>
                        <p class="mt-1 text-sm text-gray-400">{ "Active Developers" }</p>
                    </div>
                </div>

                <div class="col-span-1">
                    <div class="rounded-2xl bg-gray-200 dark:bg-gray-600/30 backdrop-blur-sm ring-1 ring-white/10 p-6 transition-all hover:bg-gray-300 dark:hover:bg-gray-600/50 duration-300">
                        <h3 class="text-sm font-semibold text-gray-400">{ "Growing Library" }</h3>
                        <p class="mt-2 text-2xl font-medium bg-gradient-to-r text-[#E54D5B] bg-clip-text text-transparent">{ "Master Your Tools" }</p>
                        <p class="mt-1 text-sm text-gray-400">{ "New content added weekly" }</p>
                    </div>
                </div>

                <div class="col-span-1">
                    <div class="col-span-1 lg:col-span-1">
                        <a href="/reviews/happy-hacking-keyboard-type-s-review">
                            <div class="rounded-2xl bg-gradient-to-br from-gray-700/40 to-gray-600/30 backdrop-blur-sm ring-1 ring-white/10 overflow-hidden transition-all hover:bg-gray-700/60 duration-300 group">
                                <img class="w-full h-32 object-cover transition-all duration-300 group-hover:scale-105" src="https://typecraft.dev/rails/active_storage/representations/redirect/eyJfcmFpbHMiOnsiZGF0YSI6OTAsInB1ciI6ImJsb2JfaWQifX0=--afa40c042b263f48189198662dc590a977ed85da/eyJfcmFpbHMiOnsiZGF0YSI6eyJmb3JtYXQiOiJwbmciLCJyZXNpemVfdG9fbGltaXQiOls0MDAsNDAwXX0sInB1ciI6InZhcmlhdGlvbiJ9fQ==--bb8bf42cd8dab2097a8e332d051d5d0fee03c71e/Screenshot-2024-10-30-at-2.40.28-PM.png" />
                                <div class="p-6 bg-gray-800/50">
                                    <span class="inline-flex items-center rounded-full bg-white px-2 py-1 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20">
                                        { "PUBLIC" }
                                    </span>
                                    <p class="mt-2 text-lg font-medium text-white">{ "Happy Hacking Keyboard Type-S Review" }</p>
                                    <p class="mt-2 text-sm text-gray-400 line-clamp-2">{ "This keyboard sounds too good to not type on." }</p>
                                    <span class="mt-3 text-accent hover:text-accent/80 font-semibold inline-flex items-center">
                                        { "Read More" }
                                        <svg xmlns="http://www.w3.org/2000/svg" class="ml-2 h-5 w-5 transform transition-[transform] duration-300 ease-out group-hover:translate-x-2 group-hover:duration-300 duration-700" viewBox="0 0 20 20" fill="currentColor">
                                            <path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                                        </svg>
                                    </span>
                                </div>
                            </div>
                        </a></div>

                </div>
                <div class="col-span-1">
                    <div class="col-span-1 lg:col-span-1">
                        <a href="/tutorial/how-well-do-you-know-vim-registers">
                            <div class="rounded-2xl bg-gradient-to-br from-gray-700/40 to-gray-600/30 backdrop-blur-sm ring-1 ring-white/10 overflow-hidden transition-all hover:bg-gray-700/60 duration-300 group">
                                <img class="w-full h-32 object-cover transition-all duration-300 group-hover:scale-105" src="https://typecraft.dev/rails/active_storage/representations/redirect/eyJfcmFpbHMiOnsiZGF0YSI6MTE1LCJwdXIiOiJibG9iX2lkIn19--005d7083d27c3ed7a2df42e1f124506ee0eb0ef7/eyJfcmFpbHMiOnsiZGF0YSI6eyJmb3JtYXQiOiJwbmciLCJyZXNpemVfdG9fbGltaXQiOls0MDAsNDAwXX0sInB1ciI6InZhcmlhdGlvbiJ9fQ==--bb8bf42cd8dab2097a8e332d051d5d0fee03c71e/registers-1.png" />
                                <div class="p-6 bg-gray-800/50">
                                    <span class="inline-flex items-center rounded-full bg-white px-2 py-1 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20">
                                        { "PUBLIC" }
                                    </span>
                                    <p class="mt-2 text-lg font-medium text-white">{ "How Well Do You Know Vim Registers?" }</p>
                                    <p class="mt-2 text-sm text-gray-400 line-clamp-2">{ "Registers are like books on a shelf — a place where text is stored..." }</p>
                                    <span class="mt-3 text-accent hover:text-accent/80 font-semibold inline-flex items-center">
                                        { "Read More" }
                                        <svg xmlns="http://www.w3.org/2000/svg" class="ml-2 h-5 w-5 transform transition-[transform] duration-300 ease-out group-hover:translate-x-2 group-hover:duration-300 duration-700" viewBox="0 0 20 20" fill="currentColor">
                                            <path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                                        </svg>
                                    </span>
                                </div>
                            </div>
                        </a></div>

                </div>
            </div>
        </>
    }
}
