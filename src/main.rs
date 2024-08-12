use std::fs;
use db::{create_blame_table, insert_blame, Person};
use poise::serenity_prelude::{self as serenity, Message};
use tokio::sync::Mutex;
use rusqlite::{Connection, Result};

mod db;

struct Data {

} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


pub fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next_back();
    chars.next_back();
    

    return chars.as_str();
}

#[poise::command(slash_command, prefix_command)]
async fn test(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn deafen(
    ctx: Context<'_>,
) -> Result<(), Error> {
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
        },
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
  
#[poise::command(slash_command, prefix_command, rename = "quit")]
async fn quit_bot(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let guild = ctx.guild().unwrap().name.clone();
    
    let channel = ctx.guild_channel().await.unwrap().name().to_string();

    ctx.reply(format!("Quitting program.")).await?;
    println!(format!("[{}: {}]: Quitting program."), guild, channel)
    ctx.framework().shard_manager.shutdown_all().await;
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

#[poise::command(slash_command, prefix_command, subcommands("add", "list"))]
async fn blame(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("It's {}'s fault, fuck that person in particular!", u.name);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {

    let guild = ctx.guild().unwrap().name.clone();
    
    let channel = ctx.guild_channel().await.unwrap().name().to_string();

    let guild_id = ctx.guild_id().unwrap().to_string();

    let _ = fs::create_dir_all(format!("./db/{}", guild_id));

    let mut conn = Mutex::new(Connection::open(format!("./db/{}/blame.db3", guild_id))?);

    let mut response = format!("The current rotation is: ");

    let _ = create_blame_table(&mut conn).await?;
    
    let names: Vec<String> = db::get_blame_list(&mut conn).await.unwrap();

    let mut names_string = String::new();

    for name in names {
        names_string.push_str(&(name.to_owned() + ", "));
        // ctx.say(name).await?;
    };



    response.push_str(rem_last(&names_string));

    println!("[{}: {}]: {}", guild, channel, response.clone());

    ctx.say(response).await?;

    Ok(())

}

#[poise::command(prefix_command, slash_command)]
async fn add(ctx: Context<'_>, 
    #[description = "Selected user"] name: String,
    #[description = "Place in rotation"] id: i32,
) -> Result<(), Error> {
 
    let guild_id = ctx.guild_id().unwrap().to_string();
  
    let guild = ctx.guild().unwrap().name.clone();
  
    let channel = ctx.guild_channel().await.unwrap().name().to_string();
 
    let _ = fs::create_dir_all(format!("./db/{}", guild_id));
    
    let mut conn = Mutex::new(Connection::open(format!("./db/{}/blame.db3", guild_id))?);

    let _ = create_blame_table(&mut conn).await?;

        let output = db::query_blame_table(&mut conn);

        let (_names, ids): (Vec<String>, Vec<i32>) = output.await.unwrap();

        for _id in ids {
            if _id == id {
                let response: String = format!("You've already added someone with the rotation id of: {}", id);
                println!("[{}: {}]: {}", guild, channel, response.clone());
                ctx.say(response).await?;               

                return Ok(());
            }
        }
    
    let person = Person {
        name: name,
        rotation_id: id,
    };

    insert_blame(&mut conn, person.clone()).await?;

    let response: String = format!("Added {} into the rotation, with placement {}.", &person.name, &person.rotation_id);
    println!("[{}: {}]: {}", guild, channel, response.clone());
    ctx.say(response).await?;
    Ok(())
}


#[tokio::main]
async fn main() { 
    dotenvy::dotenv().unwrap();

    let _ = fs::create_dir_all("./db");

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment.");
    // let token: String = fs::read_to_string("./.env").expect("Unable to read file.");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_VOICE_STATES;
    //serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), blame() ,quit_bot(), test(), deafen(),],
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
