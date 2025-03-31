use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let _link_classes = "text-gray-900 dark:text-white text-xl text-extrabold hover:text-red-400 hover:scale-110 px-1 py-2 rounded transition duration-300 ease-in-out";
    let links = [
        ("Home", "/"),
        ("Tutorials", "/tutorials"),
        ("About", "/about"),
        ("Contact", "/contact"),
    ];
    html! {
    <>
    <header class="fixed w-full z-40 top-0 min-h-20 lg:h-20 flex flex-wrap flex-none items-center backdrop-blur transition-colors duration-300 bg-gray-200 dark:bg-gray-900/80 hover:bg-gray-900/60 border-b border-gray-50/[0.02]">
        <div class="absolute inset-0 bg-gradient-to-r from-gray-900/75 to-transparent transition-opacity duration-300" data-header-scroll-target="overlay" style="opacity: 0.361;"></div>
            <nav class="fixed w-full top-0 z-50 outer xl:outer-xl flex items-center py-4 group-[.has-breadcrumbs]:lg:min-h-0 group-[.has-breadcrumbs]:min-h-20 bg-navy">
                    <div class="inner flex items-center min-h-10">
                        <a class="flex-shrink mr-8 ml-20 font-bold text-lg text-gray-900 dark:text-gray-50 has-logo lg:min-w-logo" aria-label="CodeOps HQ" href="/">
                            <img alt="CodeOps HQ" class="max-h-9 w-auto hidden dark:block" src="assets/svg/codeops-hq-side-logo.svg" />
                            <img alt="CodeOps HQ" class="max-h-9 w-auto block dark:hidden" src="assets/svg/codeops-hq-side-logo.svg" />
                        </a>
                        <ul data-navigation-desktop="" class="navigation hidden lg:flex space-x-2 text-gray-700 dark:text-gray-200" role="list">
                            {for links.iter().map(|(label, href)| html! {
                            <li data-slug="courses" class="relative inline-flex max-3xl:[&amp;:nth-child(n+6)]:hidden [&amp;:nth-child(n+6)]:hidden">
                                <a class="[&amp;amp;.external>svg]:inline px-3 py-1.5 inline-flex items-center font-semibold text-sm leading-6 whitespace-nowrap rounded-lg hover:bg-gray-100 [&amp;amp;.current]:bg-gray-100 [&amp;amp;.current-parent]:bg-gray-100 dark:hover:bg-gray-700 dark:[&amp;amp;.current]:bg-gray-700 dark:[&amp;amp;.current-parent]:bg-gray-700 " href={*href}>{label}</a>
                                <svg class="hidden flex-shrink-0 w-4 h-4 ml-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" focusable="false" aria-hidden="true">
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M17 7l-10 10"></path>
                                    <path d="M8 7l9 0l0 9"></path>
                                </svg>
                            </li>
                            })}
                        </ul>
                        <div data-header-buttons="" class="hidden lg:flex items-center justify-between flex-1 mx-5 pl-5 space-x-1 border-l text-gray-400 border-gray-900/10 dark:border-gray-50/10">
                            <button data-action="search#open" class="h-9 w-9 flex items-center justify-center flex-shrink-0 rounded-lg hover:bg-gray-100 hover:text-gray-700 dark:hover:bg-gray-700 dark:hover:text-gray-300" title="Search" aria-label="Search">
                                <svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11ZM2 9a7 7 0 1 1 12.452 4.391l3.328 3.329a.75.75 0 1 1-1.06 1.06l-3.329-3.328A7 7 0 0 1 2 9Z" clip-rule="evenodd"></path>
                                </svg>
                            </button>

                            <div class="flex items-center">
                                <a data-social-link="" class="[&amp;:nth-child(n_+_7)]:hidden h-9 w-9 flex items-center justify-center flex-shrink-0 rounded-lg hover:bg-gray-100 hover:text-gray-700 dark:hover:bg-gray-700 dark:hover:text-gray-300" href="https://twitter.com/typecraft_dev" title="Twitter" aria-label="Twitter" target="_blank">
                                    <svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="currentColor" focusable="false" aria-hidden="true">
                                        <path d="M23.954 4.569c-.885.389-1.83.654-2.825.775 1.014-.611 1.794-1.574 2.163-2.723-.951.555-2.005.959-3.127 1.184-.896-.959-2.173-1.559-3.591-1.559-2.717 0-4.92 2.203-4.92 4.917 0 .39.045.765.127 1.124C7.691 8.094 4.066 6.13 1.64 3.161c-.427.722-.666 1.561-.666 2.475 0 1.71.87 3.213 2.188 4.096-.807-.026-1.566-.248-2.228-.616v.061c0 2.385 1.693 4.374 3.946 4.827-.413.111-.849.171-1.296.171-.314 0-.615-.03-.916-.086.631 1.953 2.445 3.377 4.604 3.417-1.68 1.319-3.809 2.105-6.102 2.105-.39 0-.779-.023-1.17-.067 2.189 1.394 4.768 2.209 7.557 2.209 9.054 0 13.999-7.496 13.999-13.986 0-.209 0-.42-.015-.63.961-.689 1.8-1.56 2.46-2.548l-.047-.02z"></path>
                                    </svg>
                                </a>
                            </div>

                            <div class="flex items-center">
                                <a data-turbo-stream="true" class="text-sm font-semibold px-4 py-2 rounded-lg bg-accent text-white hover:bg-accent/90 transition-colors" href="/user_sessions/new">{"Sign in"}</a>
                            </div>
                        </div>
                    </div>
                    <div class="flex lg:hidden items-center ml-auto -mr-1.5 space-x-2 text-gray-400">
                        <button data-mobile-menu-toggle="" class="w-8 h-8 flex items-center justify-center" title="Toggle menu" aria-label="Toggle menu" data-action="click->slideover#open">
                            <svg class="w-5 h-5 hover:text-gray-700 dark:hover:text-gray-100" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" focusable="false" aria-hidden="true">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 8h16M4 16h16"></path>
                            </svg>
                        </button>
                    </div>
            </nav>
            <div class="h-20"></div>
    </header>
    </>
    }
}
