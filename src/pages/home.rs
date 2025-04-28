use yew::prelude::*;

use crate::api::youtube::latest_video::LatestVideo;

#[derive(Clone, PartialEq)]
struct Card {
    title: String,
    description: String,
    image_url: String,
    link: String,
    tag: String,
}

#[derive(Clone, PartialEq)]
struct Tool {
    name: String,
    description: String,
    icon: String,
    link: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    // Example dynamic data for cards
    let cards = vec![
        Card {
            title: "Happy Hacking Keyboard Type-S Review".to_string(),
            description: "This keyboard sounds too good to not type on.".to_string(),
            image_url: "https://typecraft.dev/rails/active_storage/representations/redirect/eyJfcmFpbHMiOnsiZGF0YSI6OTAsInB1ciI6ImJsb2JfaWQifX0=--afa40c042b263f48189198662dc590a977ed85da/eyJfcmFpbHMiOnsiZGF0YSI6eyJmb3JtYXQiOiJwbmciLCJyZXNpemVfdG9fbGltaXQiOls0MDAsNDAwXX0sInB1ciI6InZhcmlhdGlvbiJ9fQ==--bb8bf42cd8dab2097a8e332d051d5d0fee03c71e/Screenshot-2024-10-30-at-2.40.28-PM.png".to_string(),
            link: "/reviews/happy-hacking-keyboard-type-s-review".to_string(),
            tag: "PUBLIC".to_string(),
        },
        Card {
            title: "How Well Do You Know Vim Registers?".to_string(),
            description: "Registers are like books on a shelf — a place where text is stored...".to_string(),
            image_url: "https://typecraft.dev/rails/active_storage/representations/redirect/eyJfcmFpbHMiOnsiZGF0YSI6MTE1LCJwdXIiOiJibG9iX2lkIn19--005d7083d27c3ed7a2df42e1f124506ee0eb0ef7/eyJfcmFpbHMiOnsiZGF0YSI6eyJmb3JtYXQiOiJwbmciLCJyZXNpemVfdG9fbGltaXQiOls0MDAsNDAwXX0sInB1ciI6InZhcmlhdGlvbiJ9fQ==--bb8bf42cd8dab2097a8e332d051d5d0fee03c71e/registers-1.png".to_string(),
            link: "/tutorial/how-well-do-you-know-vim-registers".to_string(),
            tag: "PUBLIC".to_string(),
        },
    ];

    // Example dynamic data for tools
    let tools = vec![
        Tool {
            name: "Neovim".to_string(),
            description: "A highly configurable text editor built on Vim.".to_string(),
            icon: "assets/svg/neovim.svg".to_string(),
            link: "https://neovim.io/".to_string(),
        },
        Tool {
            name: "Git".to_string(),
            description: "Distributed version control system.".to_string(),
            icon: "assets/svg/git.svg".to_string(),
            link: "https://git-scm.com/".to_string(),
        },
        Tool {
            name: "Tmux".to_string(),
            description: "A terminal multiplexer and interactive session manager.".to_string(),
            icon: "assets/svg/tmux.svg".to_string(),
            link: "https://tmux.github.io/".to_string(),
        },
        Tool {
            name: "Rust Programming Language".to_string(),
            description: "A systems programming language focused on safety and performance."
                .to_string(),
            icon: "assets/svg/rust.svg".to_string(),
            link: "https://www.rust-lang.org/".to_string(),
        },
        Tool {
            name: "GitHub".to_string(),
            description: "A code hosting platform for version control and collaboration."
                .to_string(),
            icon: "assets/svg/github-2.svg".to_string(),
            link: "https://github.com/".to_string(),
        },
        Tool {
            name: "GitLab".to_string(),
            description: "A code hosting platform for version control and collaboration."
                .to_string(),
            icon: "assets/svg/gitlab.svg".to_string(),
            link: "https://gitlab.com/".to_string(),
        },
        Tool {
            name: "Nginx".to_string(),
            description: "A web server and reverse proxy.".to_string(),
            icon: "assets/svg/nginx.svg".to_string(),
            link: "https://www.nginx.com/".to_string(),
        },
        //Tool {
        //    name: "Visual Studio Code".to_string(),
        //    description: "A lightweight but powerful source code editor.".to_string(),
        //    icon: "https://upload.wikimedia.org/wikipedia/commons/thumb/9/9a/Visual_Studio_Code_1.35_icon.svg/1200px-Visual_Studio_Code_1.35_icon.svg.png".to_string(),
        //    link: "https://code.visualstudio.com/".to_string(),
        //},
        //Tool {
        //    name: "GitHub Copilot".to_string(),
        //    description: "AI-powered coding assistant for faster development.".to_string(),
        //    icon: "https://github.githubassets.com/images/modules/site/features/copilot-icon.svg".to_string(),
        //    link: "https://copilot.github.com/".to_string(),
        //},
        Tool {
            name: "Postman".to_string(),
            description: "API development environment for testing and collaboration.".to_string(),
            icon: "https://www.vectorlogo.zone/logos/getpostman/getpostman-icon.svg".to_string(),
            link: "https://www.postman.com/".to_string(),
        },
        Tool {
            name: "Docker".to_string(),
            description: "A platform for building, shipping, and running distributed applications."
                .to_string(),
            icon: "https://www.vectorlogo.zone/logos/docker/docker-icon.svg".to_string(),
            link: "https://www.docker.com/".to_string(),
        },
    ];

    html! {
        <>
            <section class="flex py-16 items-center justify-center bg-gray-100 dark:bg-gray-900">
                <h2 class="text-5xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-green-400 to-blue-500">
                    {"Our Latest Video"}
                </h2>
            </section>

            <section class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 p-4">
                // Featured Video Section
                <div class="col-span-1 md:col-span-2 lg:col-span-2 row-span-2">
                    <LatestVideo />
                </div>

                // Stats Section
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

                // Dynamic Cards Section
                { for cards.iter().map(|card| {
                    html! {
                        <div class="col-span-1">
                            <a href={card.link.clone()}>
                                <div class="rounded-2xl bg-gradient-to-br from-gray-700/40 to-gray-600/30 backdrop-blur-sm ring-1 ring-white/10 overflow-hidden transition-all hover:bg-gray-700/60 duration-300 group">
                                    <img class="w-full h-32 object-cover transition-all duration-300 group-hover:scale-105" src={card.image_url.clone()} alt={card.title.clone()} />
                                    <div class="p-6 bg-gray-800/50">
                                        <span class="inline-flex items-center rounded-full bg-white px-2 py-1 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20">
                                            { &card.tag }
                                        </span>
                                        <p class="mt-2 text-lg font-medium text-white">{ &card.title }</p>
                                        <p class="mt-2 text-sm text-gray-400 line-clamp-2">{ &card.description }</p>
                                        <span class="mt-3 text-accent hover:text-accent/80 font-semibold inline-flex items-center">
                                            { "Read More" }
                                            <svg xmlns="http://www.w3.org/2000/svg" class="ml-2 h-5 w-5 transform transition-[transform] duration-300 ease-out group-hover:translate-x-2 group-hover:duration-300 duration-700" viewBox="0 0 20 20" fill="currentColor">
                                                <path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                                            </svg>
                                        </span>
                                    </div>
                                </div>
                            </a>
                        </div>
                    }
                }) }
            </section>

            // New Section: Essential Developer Tools
            <section class="py-16 bg-gray-100 dark:bg-gray-900">
                <div class="container mx-auto px-4">
                    <h2 class="text-4xl font-extrabold text-center text-gray-800 dark:text-white mb-8">{"Essential Developer Tools"}</h2>
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
                        { for tools.iter().map(|tool| {
                            html! {
                                <div class="rounded-2xl bg-white dark:bg-gray-800 shadow-lg p-6 transition-all hover:shadow-xl hover:-translate-y-1 duration-300">
                                    <img class="w-16 h-16 mx-auto mb-4" src={tool.icon.clone()} alt={tool.name.clone()} />
                                    <h3 class="text-lg font-bold text-center text-gray-800 dark:text-white">{ &tool.name }</h3>
                                    <p class="mt-2 text-sm text-center text-gray-600 dark:text-gray-400">{ &tool.description }</p>
                                    <a href={tool.link.clone()} target="_blank" class="mt-4 block text-center text-blue-500 hover:text-blue-700 font-medium">
                                        { "Learn More" }
                                    </a>
                                </div>
                            }
                        }) }
                    </div>
                </div>
            </section>
        </>
    }
}
