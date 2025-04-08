use std::env;

fn main() {
    // Load variables from .env file in the project root
    // This will fail compilation if .env is not found, which is often desired.
    dotenvy::dotenv().expect("Failed to load .env file for build script");

    // Iterate over environment variables provided by dotenvy
    // and expose them to the main crate compilation
    for (key, value) in env::vars() {
        // IMPORTANT: Decide which variables to expose.
        // Avoid exposing truly sensitive secrets meant only for backend/build process.
        // You might want a specific prefix or allowlist.
        // Here, we expose API_URL and SECRET_BUILD_INFO as an example.
        //if key == "API_URL" || key == "YOUTUBE_API_KEY" {
        println!("cargo:rustc-env={}={}", key, value);
        //}
    }

    // Tell cargo to rerun the build script if .env changes.
    println!("cargo:rerun-if-changed=.env");
}
