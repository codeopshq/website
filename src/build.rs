use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // Load environment variables from .env
    dotenv().ok();

    // Get the API key
    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY not found in .env file");

    // Write the key to a generated Rust module
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("env.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "pub const YOUTUBE_API_KEY: &str = \"{}\";", api_key).unwrap();
}
