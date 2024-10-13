use axum::{
    routing::{get, post},
    Router,
};
use shuttle_runtime::SecretStore;

mod discord;

#[derive(Clone)]
struct AppState {
    discord_info: discord::DiscordInfo,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    let info = discord::DiscordInfo {
        bot_api_key: secrets.get("DISCORD_BOT_TOKEN").unwrap(),
        guild_id: secrets.get("DISCORD_GUILD_ID").unwrap(),
        channel_name: secrets.get("CHANNEL_NAME").unwrap(),
    };

    let state = AppState { discord_info: info };
    // println!("Discord info: {:?}", state.discord_info);

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/discord", post(discord::discord::send_file))
        .with_state(state);

    Ok(router.into())
}
