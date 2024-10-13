pub mod discord;

#[derive(Clone, Debug)]
pub struct DiscordInfo {
    pub bot_api_key: String,
    pub guild_id: String,
    pub channel_name: String,
}
