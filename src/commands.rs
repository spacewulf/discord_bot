use std::fs;
use poise::serenity_prelude::{self as serenity};
use tokio::sync::Mutex;
use rusqlite::{Connection, Result};
use songbird::input::YoutubeDl;
