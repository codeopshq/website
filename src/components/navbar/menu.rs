use web_sys::window;
use yew::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    let link_classes = "[&>.external>svg]:inline px-3 py-1.5 inline-flex items-center font-semibold text-sm leading-6 whitespace-nowrap rounded-lg hover:bg-blue-100 [&.current]:bg-blue-100 [&.current-parent]:bg-blue-100 dark:hover:bg-gray-700 dark:[&.current]:bg-gray-700 dark:[&.current-parent]:bg-gray-700 hover:scale-105 duration-300 ease-in-out";

    let links = [
        ("Home", "/"),
        ("Tutorials", "/tutorials"),
        ("About", "/about"),
        ("Contact", "/contact"),
    ];

    // Get current path
    let current_path = window()
        .and_then(|win| win.location().pathname().ok())
        .unwrap_or_default();

    html! {
        <ul data-navigation-desktop="" class="navigation hidden lg:flex space-x-2 text-gray-700 dark:text-gray-200" role="list">
            {for links.iter().map(|(label, href)| {
                let is_current = current_path == *href;
                let is_parent = *href != "/" && current_path.starts_with(href);

                let mut classes = Classes::from(link_classes);
                if is_current {
                    classes.push("current");
                } else if is_parent {
                    classes.push("current-parent");
                }
            html! {
            <li data-slug="courses" class="relative inline-flex max-3xl:[&amp;:nth-child(n+6)]:hidden [&amp;:nth-child(n+6)]:hidden">
                <a class={classes} href={*href}>{label}</a>
                <svg class="hidden flex-shrink-0 w-4 h-4 ml-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" focusable="false" aria-hidden="true">
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M17 7l-10 10"></path>
                    <path d="M8 7l9 0l0 9"></path>
                </svg>
            </li>
            }
            })}
        </ul>
    }
}
