use crate::components::navbar::{header_button::HeaderButton, menu::Menu};
use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <>
            <header data-header="" class="fixed w-full z-40 top-0 min-h-20 lg:h-20 flex flex-wrap flex-none items-center backdrop-blur transition-colors duration-300 bg-gray-900/40 hover:bg-gray-900/60 border-b border-gray-50/[0.02]" data-header-scroll-target="header" style="background-color: rgba(17, 24, 39, 0.1);">
                <div class="absolute inset-0 bg-gradient-to-r from-gray-900/75 to-transparent transition-opacity duration-300" data-header-scroll-target="overlay" style="opacity: 0;"></div>
                <nav class="fixed w-full top-0 z-50 outer xl:outer-xl flex items-center py-4 group-[.has-breadcrumbs]:lg:min-h-0 group-[.has-breadcrumbs]:min-h-20 bg-navy">
                    <div class="inner flex items-center min-h-10 w-full mx-20">
                        <a class="flex-shrink font-bold text-lg text-gray-900 dark:text-gray-50 has-logo lg:min-w-logo" aria-label="CodeOps HQ" href="/">
                            <img alt="CodeOps HQ" class="max-h-9 w-auto hidden dark:block" src="assets/svg/codeops-hq-side-logo.svg" />
                            <img alt="CodeOps HQ" class="max-h-9 w-auto block dark:hidden" src="assets/svg/codeops-hq-side-logo.svg" />
                        </a>
                        <HeaderButton />
                        <Menu />
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
