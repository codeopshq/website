use gloo_console::log; // For logging form data in the placeholder
                       //use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*; // For cloning state into callbacks easily
                     // Add this if you removed it previously, for the timer
use gloo_timers::future::TimeoutFuture;

// Represents the state of the contact form
#[derive(Clone, Default, PartialEq, Debug)]
struct ContactFormState {
    name: String,
    email: String,
    subject: String,
    message: String,
}

// Represents the status after attempting submission
#[derive(Clone, PartialEq)]
enum SubmissionStatus {
    Idle,
    Submitting,
    Success,
    Error(String),
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let form_state = use_state(ContactFormState::default);
    let submission_status = use_state(|| SubmissionStatus::Idle);

    // --- Input Handlers (No changes needed) ---
    let on_input = {
        let form_state = form_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let name = input.name();
            let value = input.value();
            let mut state = (*form_state).clone();
            match name.as_str() {
                "name" => state.name = value,
                "email" => state.email = value,
                "subject" => state.subject = value,
                _ => {}
            }
            form_state.set(state);
        })
    };
    let on_textarea_input = {
        let form_state = form_state.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            let value = textarea.value();
            let mut state = (*form_state).clone();
            state.message = value;
            form_state.set(state);
        })
    };

    // --- Form Submission Handler (Placeholder - No changes needed) ---
    let on_submit = {
        let form_state = form_state.clone();
        let submission_status = submission_status.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let current_form_data = (*form_state).clone();
            let submission_status_clone = submission_status.clone();

            if current_form_data.name.is_empty()
                || current_form_data.email.is_empty()
                || current_form_data.message.is_empty()
            {
                submission_status_clone.set(SubmissionStatus::Error(
                    "Please fill in all required fields.".to_string(),
                ));
                return;
            }
            submission_status.set(SubmissionStatus::Submitting);

            spawn_local(async move {
                TimeoutFuture::new(1_000).await; // Simulate delay
                log!(format!(
                    "Form Data Submitted (Placeholder): {:?}",
                    current_form_data
                ));
                let success = true; // Simulate success
                if success {
                    submission_status_clone.set(SubmissionStatus::Success);
                } else {
                    submission_status_clone.set(SubmissionStatus::Error(
                        "Submission failed. Please try again.".to_string(),
                    ));
                }
            });
        })
    };

    // --- Minimal HTML Structure ---
    html! {
        // Use subtle page background
        <div class="min-h-screen py-12 md:py-16 lg:py-20 px-4 sm:px-6 lg:px-8">
            <div class="container mx-auto max-w-4xl"> // Reduced max-width slightly

                // --- Page Header (Simplified) ---
                <div class="text-center mb-10 md:mb-12">
                    <h1 class="text-3xl sm:text-4xl font-semibold tracking-tight text-gray-900 dark:text-white">
                        { "Contact Us" }
                    </h1>
                    <p class="mt-3 text-lg text-gray-600 dark:text-gray-400">
                        { "Have questions? We'd love to hear from you." }
                    </p>
                </div>

                // --- Main Content Grid (Info + Form) ---
                // Increased gap for more whitespace
                <div class="grid grid-cols-1 md:grid-cols-2 gap-10 lg:gap-16">

                    // --- Contact Information Section (Simplified, no card/icons) ---
                    <div class="space-y-6"> // Use space-y for vertical spacing
                         // Email
                        <div>
                            <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">{ "Email" }</h3>
                            <a href="mailto:hello@codeops.com" class="mt-1 text-lg text-gray-900 dark:text-white hover:text-gray-700 dark:hover:text-gray-300">
                                { "hello@codeops.com" } // Replace
                            </a>
                        </div>
                        // sponsership
                        // only for sponsership email
                        <div>
                            <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">{ "sponsership" }</h3>
                            <h2 class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">{ "only for sponsership" }</h2>
                            <a href="mailto:codeopshq@gmail.com" class="mt-1 text-lg text-gray-900 dark:text-white hover:text-gray-700 dark:hover:text-gray-300">
                                { "codeopshq@gmail.com" } // Replace
                            </a>
                        </div>
                         // Phone
                        <div>
                            <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">{ "Phone" }</h3>
                            <a href="tel:+251714623651" class="mt-1 text-lg text-gray-900 dark:text-white hover:text-gray-700 dark:hover:text-gray-300">
                                { "+251 971 46 23 651" } // Replace
                            </a>
                        </div>
                        // Address
                        <div>
                            <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">{ "Address" }</h3>
                            <p class="mt-1 text-lg text-gray-900 dark:text-white">
                                { "054 Nadhi Gamada Street, Adama, Oromia, Ethiopia" } // Replace
                            </p>
                        </div>
                         // Add Social Media Links here if desired, styled minimally
                    </div>

                    // --- Contact Form Section (Simplified, no card) ---
                    <div>
                        // Removed the explicit heading, form is self-explanatory
                        <form onsubmit={on_submit} class="space-y-5"> // Slightly reduced space-y
                            // Name Input
                            <div>
                                <label for="name" class="sr-only">{ "Full Name" }</label> // Screen-reader only label
                                <input type="text" name="name" id="name" required=true
                                       class="block w-full px-4 py-3 border border-gray-300 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900 text-gray-900 dark:text-white focus:border-gray-500 focus:ring-gray-500 sm:text-sm"
                                       placeholder="Full Name" // Use placeholder text
                                       value={form_state.name.clone()}
                                       oninput={on_input.clone()}
                                />
                            </div>
                            // Email Input
                            <div>
                                <label for="email" class="sr-only">{ "Email Address" }</label>
                                <input type="email" name="email" id="email" required=true
                                       class="block w-full px-4 py-3 border border-gray-300 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900 text-gray-900 dark:text-white focus:border-gray-500 focus:ring-gray-500 sm:text-sm"
                                       placeholder="Email Address"
                                       value={form_state.email.clone()}
                                       oninput={on_input.clone()}
                                />
                            </div>
                            // Subject Input
                            <div>
                                <label for="subject" class="sr-only">{ "Subject" }</label>
                                <input type="text" name="subject" id="subject"
                                       class="block w-full px-4 py-3 border border-gray-300 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900 text-gray-900 dark:text-white focus:border-gray-500 focus:ring-gray-500 sm:text-sm"
                                       placeholder="Subject (Optional)"
                                       value={form_state.subject.clone()}
                                       oninput={on_input.clone()}
                                />
                            </div>
                            // Message Textarea
                            <div>
                                <label for="message" class="sr-only">{ "Message" }</label>
                                <textarea name="message" id="message" rows="5" required=true // Increased rows slightly
                                          class="block w-full px-4 py-3 border border-gray-300 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900 text-gray-900 dark:text-white focus:border-gray-500 focus:ring-gray-500 sm:text-sm"
                                          placeholder="Your Message"
                                          value={form_state.message.clone()}
                                          oninput={on_textarea_input}
                                ></textarea>
                            </div>

                            // Submission Button & Status Messages
                            <div class="flex flex-col items-start"> // Align button and messages left
                                <button type="submit"
                                        disabled={*submission_status == SubmissionStatus::Submitting}
                                        // Minimal button style
                                        class="inline-flex justify-center py-3 px-6 border border-transparent shadow-sm text-base font-medium rounded-md text-white bg-gray-800 hover:bg-gray-700 dark:bg-gray-200 dark:text-gray-900 dark:hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 dark:focus:ring-offset-black disabled:opacity-50 transition ease-in-out duration-150">
                                    { if *submission_status == SubmissionStatus::Submitting { "Sending..." } else { "Send Message" } }
                                </button>

                                // Display Success/Error Messages
                                <div class="mt-3 h-5"> // Reserve space for message to prevent layout shift
                                    { match &*submission_status {
                                        SubmissionStatus::Success => html! { <p class="text-sm text-green-600 dark:text-green-400">{ "Message sent successfully!" }</p> },
                                        SubmissionStatus::Error(msg) => html! { <p class="text-sm text-red-600 dark:text-red-400">{ msg }</p> },
                                        _ => html! {} // Idle or Submitting
                                    }}
                                </div>
                                // Keep note about backend, but make it subtle
                                // <p class="mt-2 text-xs text-gray-400 dark:text-gray-500">
                                //     { "Note: Form submission requires a backend service." }
                                // </p>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
