use chrono::prelude::*;
use poise::serenity_prelude::{self as serenity};
use rusqlite::{Connection, Result};
use std::fs;
use tokio::sync::Mutex;

use crate::db;
use crate::methods;
use crate::structs::{Data, Person};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command, rename = "quit")]
pub async fn quit_bot(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().name.clone();

    let channel = ctx.guild_channel().await.unwrap().name().to_string();

    ctx.reply("Quitting program.".to_string()).await?;
    println!("[{}: {}]: Quitting program.", guild, channel);
    ctx.framework().shard_manager.shutdown_all().await;
    Ok(())
}

#[poise::command(slash_command, prefix_command, subcommands("add", "list", "get"))]
pub async fn blame(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("It's {}'s fault, fuck that person in particular!", u.name);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn add(
    ctx: Context<'_>,
    #[description = "Selected user"] name: String,
    #[description = "Place in rotation"] id: i32,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().to_string();

    let guild = ctx.guild().unwrap().name.clone();

    let channel = ctx.guild_channel().await.unwrap().name().to_string();

    let _ = fs::create_dir_all(format!("./db/{}", guild_id));

    let mut conn = Mutex::new(Connection::open(format!("./db/{}/blame.db3", guild_id))?);

    db::create_blame_table(&mut conn).await?;

    let output = db::query_blame_table(&mut conn);

    let (_names, ids): (Vec<String>, Vec<i32>) = output.await.unwrap();

    for _id in ids {
        if _id == id {
            let response: String = format!(
                "You've already added someone with the rotation id of: {}",
                id
            );
            println!("[{}: {}]: {}", guild, channel, response.clone());
            ctx.say(response).await?;

            return Ok(());
        }
    }
    let person = Person {
        name: name,
        rotation_id: id,
    };
    db::insert_blame(&mut conn, person.clone()).await?;
    let response: String = format!(
        "Added {} into the rotation, with placement {}.",
        &person.name, &person.rotation_id
    );
    println!("[{}: {}]: {}", guild, channel, response.clone());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().name.clone();
    let guild_id = ctx.guild_id().unwrap().to_string();
    let _ = fs::create_dir_all(format!("./db/{}", guild_id));
    let mut conn = Mutex::new(Connection::open(format!("./db/{}/blame.db3", guild_id))?);
    let mut response = "The current rotation is: ".to_string();
    db::create_blame_table(&mut conn).await?;
    let names: Vec<String> = db::get_blame_list(&mut conn).await.unwrap();
    let mut names_string = String::new();
    for name in names {
        names_string.push_str(&(name.to_owned() + ", "));
        // ctx.say(name).await?;
    }

    response.push_str(methods::rem_last(&names_string));
    println!("[{}: {}]: {}", guild, channel, response.clone());
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn get(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().name.clone();
    let channel = ctx.guild_channel().await.unwrap().name().to_string();
    let guild_id = ctx.guild_id().unwrap().to_string();
    let _ = fs::create_dir_all(format!("./db/{}", guild_id));
    let mut conn = Mutex::new(Connection::open(format!("./db/{}/blame.db3", guild_id))?);
    let mut response_start = "It's ".to_string();
    let mut response_end = "'s fault, fuck that person in particular!".to_string();
    let num_blame: usize = db::get_blame_list(&mut conn).await.unwrap().len();
    let utc: DateTime<Utc> = Utc::now();

    Ok(())
}
