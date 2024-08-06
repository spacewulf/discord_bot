use std::fs;
use db::{create_blame_table, insert_blame, Person};
use poise::serenity_prelude::{self as serenity, futures::TryFutureExt};
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

// Displays your uor another user's account creation date
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

    let mut conn = Mutex::new(Connection::open("./db/blame.db3")?);

    let mut response = String::new();

    response = format!("The current rotation is: ");

    let _ = create_blame_table(&mut conn).await?;
    
    let names: Vec<String> = db::get_blame_list(&mut conn).await.unwrap();

    let mut names_string = String::new();

    for name in names {
        names_string.push_str(&(name.to_owned() + ", "));
        // ctx.say(name).await?;
    };



    response.push_str(rem_last(&names_string));

    ctx.say(response).await?;

    Ok(())

}

#[poise::command(prefix_command, slash_command)]
async fn add(ctx: Context<'_>, 
    #[description = "Selected user"] name: String,
    #[description = "Place in rotation"] id: i32,
) -> Result<(), Error> {

    let mut conn = Mutex::new(Connection::open("./db/blame.db3")?);

    let _ = create_blame_table(&mut conn).await?;

        let mut response = String::new();

        let output = db::query_blame_table(&mut conn);

        let (_names, ids): (Vec<String>, Vec<i32>) = output.await.unwrap();

        for i in ids {
            if i == id {
                response = format!("You've already added someone with the rotation id of: {}", id);
                ctx.say(response).await?;

                return Ok(());
            }
        }
    
    let person = Person {
        name: name,
        rotation_id: id,
    };

    insert_blame(&mut conn, person.clone()).await?;

    response = format!("Added {} into the rotation, with placement {}.", &person.name, &person.rotation_id);
    ctx.say(response).await?;
    Ok(())
}


#[tokio::main]
async fn main() { 

    let token: String = fs::read_to_string("./.env").expect("Unable to read file.");
    let intents = serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), blame(),],
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


    client.unwrap().start().await.unwrap();
}