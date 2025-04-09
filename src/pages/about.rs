use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    // Placeholder for the YouTube Channel ID - replace with the actual ID
    const YOUTUBE_CHANNEL_ID: &str = "@CodeOpsHQ";
    let youtube_subscribe_link = format!(
        "https://www.youtube.com/{}?sub_confirmation=1",
        YOUTUBE_CHANNEL_ID
    );

    html! {
    // Add a container with padding for overall spacing
    <div class="container mx-auto px-4 py-12 md:py-16 lg:py-20">
        // Use a background color for the card effect, with dark mode support
        <div class="rounded-xl shadow-lg overflow-hidden">
            // Responsive layout: stack columns on mobile, side-by-side on large screens
            <div class="flex flex-col lg:flex-row lg:items-center">

                // --- Image Section ---
                // Takes full width on mobile, specific width on large screens
                <div class="w-full lg:w-2/5 flex-shrink-0">
                    <img
                        class="h-64 w-full object-cover lg:h-full" // Adjust height as needed
                        // Use a placeholder, replace with your actual image path
                        src="assets/img/codeops-hq-logo.svg"
                        alt="CodeOps HQ Logo or relevant visual"
                    />
                </div>

                // --- Text Content Section ---
                // Takes full width on mobile, remaining width on large screens
                <div class="w-full lg:w-3/5 p-6 sm:p-8 md:p-10 lg:p-12">
                    // Main Title
                    <h1 class="text-3xl sm:text-4xl lg:text-5xl font-bold tracking-tight text-indigo-600 dark:text-blue-400 uppercase mb-2">
                        { "CodeOps HQ" }
                    </h1>
                    // Subtitle (using a paragraph tag now)
                    <p class="mt-1 text-xl sm:text-2xl font-semibold text-gray-800 dark:text-gray-100 mb-4">
                        { "Programming, DevOps and Linux" }
                    </p>
                    // Description Paragraph
                    <p class="mt-4 text-base sm:text-lg text-gray-600 dark:text-gray-400 leading-relaxed space-y-4">
                        { "CodeOps HQ is your go-to destination for in-depth tutorials, practical tips, and expert insights on programming, Linux, and DevOps." }
                        <br/> // Add line breaks for readability if desired
                        { "We're passionate about helping you level up your technical skills and stay ahead of the curve in the fast-paced world of technology. Whether you're a seasoned developer or just starting your coding journey, our channel offers something for everyone." }
                        <br/>
                        { "Join us as we explore the latest tools, frameworks, and best practices, and become a master of code operations!" }
                    </p>

                    // Call to Action Button (YouTube Subscribe)
                    <div class="mt-8"> // Add margin-top for spacing
                        <a
                            href={youtube_subscribe_link}
                            target="_blank" // Open in new tab
                            rel="noopener noreferrer" // Security best practice for target="_blank"
                            class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 dark:focus:ring-offset-gray-800 transition ease-in-out duration-150"
                        >
                            // Simple inline SVG for YouTube icon (replace with Lucide if preferred)
                            <svg class="w-6 h-6 mr-2" aria-hidden="true" data-prefix="fab" data-icon="youtube" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512" class="svgyoutube  ">
                                <path fill="currentColor" d="M549.655 124.083c-6.281-23.65-24.787-42.276-48.284-48.597C458.781 64 288 64 288 64S117.22 64 74.629 75.486c-23.497 6.322-42.003 24.947-48.284 48.597-11.412 42.867-11.412 132.305-11.412 132.305s0 89.438 11.412 132.305c6.281 23.65 24.787 41.5 48.284 47.821C117.22 448 288 448 288 448s170.78 0 213.371-11.486c23.497-6.321 42.003-24.171 48.284-47.821 11.412-42.867 11.412-132.305 11.412-132.305s0-89.438-11.412-132.305zm-317.51 213.508V175.185l142.739 81.205-142.739 81.201z"></path>
                            </svg>
                            { "Subscribe on YouTube" }
                        </a>
                    </div>
                </div>
            </div>
        </div>
    </div>
    }
}
