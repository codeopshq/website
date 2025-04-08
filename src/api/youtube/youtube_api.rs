use anyhow::{anyhow, Result};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub content_details: ContentDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentDetails {
    pub related_playlists: RelatedPlaylists,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedPlaylists {
    pub uploads: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PlaylistItem {
    pub snippet: Snippet,
    #[serde(rename = "contentDetails")]
    pub content_details: ContentDetailsItem,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Snippet {
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContentDetailsItem {
    #[serde(rename = "videoId")]
    pub video_id: String,
    #[serde(rename = "videoPublishedAt")]
    pub video_published_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Thumbnails {
    pub medium: Thumbnail,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Thumbnail {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistResponse {
    pub items: Vec<PlaylistItem>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

pub async fn get_channel_uploads_playlist(api_key: &str, channel_id: &str) -> Result<String> {
    let url = format!(
        "{}/channels?part=contentDetails&id={}&key={}",
        env!("YOUTUBE_API_URL"),
        channel_id,
        api_key
    );

    let response = Request::get(&url).send().await?;
    let channel_response: serde_json::Value = response.json().await?;

    let playlist_id = channel_response["items"][0]["contentDetails"]["relatedPlaylists"]["uploads"]
        .as_str()
        .ok_or(anyhow!("Playlist ID not found"))?
        .to_string();

    Ok(playlist_id)
}

pub async fn get_playlist_videos(
    api_key: &str,
    playlist_id: &str,
    page_token: Option<&str>,
) -> Result<PlaylistResponse> {
    let mut url = format!(
        "{}/playlistItems?part=snippet,contentDetails&playlistId={}&key={}&maxResults={}",
        env!("YOUTUBE_API_URL"),
        playlist_id,
        api_key,
        6
    );

    if let Some(token) = page_token {
        url.push_str(&format!("&pageToken={}", token));
    }

    let response = Request::get(&url).send().await?;
    let playlist_response: PlaylistResponse = response.json().await?;

    Ok(playlist_response)
}
