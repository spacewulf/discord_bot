use crate::structs::{Data, Person};
use poise::serenity_prelude::{self as serenity};
use rusqlite::{Connection, Result};
use std::fs;
use tokio::sync::Mutex;

mod commands;
mod db;
mod methods;
mod structs;

// pub struct Data {

// } // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn deafen(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialization")
        .clone();
    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            ctx.reply("Not in a voice channel").await?;
            return Ok(());
        }
    };
    let mut handler = handler_lock.lock().await;
    if handler.is_deaf() {
        ctx.reply("Already deafened").await?;
    } else {
        if let Err(e) = handler.deafen(true).await {
            ctx.reply(format!("Failed: {:?}", e)).await?;
        }
        ctx.reply("Deafened").await?;
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let _ = fs::create_dir_all("./db");
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment.");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_VOICE_STATES;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), commands::blame(), commands::quit_bot(), deafen()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    println!("Bot started successfully.");
    client.unwrap().start().await.unwrap();
}
