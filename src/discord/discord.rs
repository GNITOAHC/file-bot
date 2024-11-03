use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest::multipart;
use serde_json::{json, Value};

use crate::AppState;

const DISCORD_CHANNEL_API: &str = "https://discord.com/api/v10/channels/{channel_id}/messages";
const DISCORD_GUILD_API: &str = "https://discord.com/api/v10/guilds/{guild_id}/channels";

pub async fn send_file(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let discord_info = &state.discord_info;

    // Extract channel id
    let channel_id = match get_channel_id(
        &discord_info.bot_api_key,
        &discord_info.guild_id,
        &discord_info.channel_name,
    )
    .await
    {
        Ok(channel_id) => channel_id,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };

    // println!("Channel ID: {}", channel_id);

    let field = match multipart.next_field().await.unwrap() {
        Some(field) => field,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read multipart field in the request",
            )
                .into_response();
        }
    };

    let file_name = match field.file_name().map(|x| x.to_string()) {
        Some(file_name) => file_name,
        None => {
            return (StatusCode::BAD_REQUEST, "File name is missing").into_response();
        }
    };

    let data = field.bytes().await.unwrap(); // .bytes() moves the field
    match upload(&file_name, &data, &channel_id, &discord_info.bot_api_key).await {
        Ok(s) => {
            return (StatusCode::OK, Json(json!({"url": s}))).into_response();
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }
}

async fn upload(
    file_name: &str,
    data: &[u8],
    channel_id: &str,
    bot_api_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let form = multipart::Form::new().part(
        "file",
        multipart::Part::bytes(data.to_vec()).file_name(file_name.to_string()),
    );
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bot {bot_api_key}", bot_api_key = bot_api_key)
            .parse()
            .unwrap(),
    );

    let url = DISCORD_CHANNEL_API.replace("{channel_id}", channel_id);
    let response = client
        .post(&url)
        .headers(headers)
        .multipart(form)
        .send()
        .await?;

    // Check if the response was successful
    if !response.status().is_success() {
        return Err("Failed to send file".into());
    }

    let result: Value = match response.json().await {
        Ok(result) => result,
        Err(e) => return Err(e.into()),
    };

    let file_url = match result["attachments"][0]["url"].as_str() {
        Some(url) => url,
        None => return Err("Failed to get file URL".into()),
    };

    return Ok(file_url.to_string());
}

async fn get_channel_id(
    bot_api_key: &str,
    guild_id: &str,
    channel_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = DISCORD_GUILD_API.replace("{guild_id}", &guild_id);
    let client = reqwest::Client::new();

    // Create headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bot {bot_api_key}", bot_api_key = bot_api_key)
            .parse()
            .unwrap(),
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let response = client.get(&url).headers(headers).send().await?;
    let channels: Vec<serde_json::Value> = response.json().await?;
    for channel in channels {
        let cur_name = channel["name"].as_str().unwrap();
        if cur_name == channel_name {
            return Ok(channel["id"].as_str().unwrap().to_string());
        }
    }
    Err("Channel not found".into())
}
