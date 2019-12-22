#![feature(proc_macro_hygiene)]
extern crate compile_time_serde;

use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, StandardFramework,
    },
    model::channel::Message,
    prelude::*,
    utils::MessageBuilder,
};
use std::fmt;

struct Author {
    name: &'static str,
    age: u8,
    city: Option<&'static str>,
}

impl fmt::Debug for Author {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({}, {})", self.name, self.age)
    }
}

struct Game {
    title: &'static str,
    tools: Option<staticvec::StaticVec<&'static str>>, // TODO: maybe use an enum for this
    platforms: Option<staticvec::StaticVec<&'static str>>, // TODO: maybe use an enum for this
    authors: Option<staticvec::StaticVec<Author>>,
    year: i32,
    award: &'static str,
    group: &'static str,
    quote: &'static str,
    description: &'static str,
    winner: bool,
}

impl fmt::Display for Game {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "title: {}, author(s): {:?}, year: {}",
            self.title,
            self.authors.as_ref().unwrap_or(&Vec::<Author>::new()),
            self.year
        )
    }
}

const GAMES: staticvec::StaticVec<Game> = compile_time_serde::deserialize_json!();

struct Handler;

impl EventHandler for Handler {}

group!({
    name: "default",
    commands: [games],
});

fn main() {
    let mut client = Client::new(&std::env::var("DISCORD_TOKEN").unwrap(), Handler).unwrap();
    client.with_framework(
        StandardFramework::new()
            .configure(|configuration| configuration.prefix("!"))
            .group(&DEFAULT_GROUP),
    );
    client.start().unwrap();
}

#[command]
fn games(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let games = GAMES.iter().filter(|game| {
        args.restore();
        while !args.is_empty() {
            match args.single_quoted::<String>().unwrap().as_ref() {
                "author" => {
                    if let Some(authors) = &game.authors {
                        let search_author = args.single_quoted::<String>().unwrap().to_lowercase();
                        for (i, author) in authors.iter().enumerate() {
                            if author
                                .name
                                .to_lowercase()
                                .contains(search_author.as_ref() as &str)
                            {
                                break;
                            }
                            if i == authors.len() - 1 {
                                return false;
                            }
                        }
                    } else {
                        return false;
                    }
                }
                "title" => {
                    if !game.title.to_lowercase().contains(
                        args.single_quoted::<String>()
                            .unwrap()
                            .to_lowercase()
                            .as_ref() as &str,
                    ) {
                        return false;
                    }
                }
                "award" => {
                    if !game.award.to_lowercase().contains(
                        args.single_quoted::<String>()
                            .unwrap()
                            .to_lowercase()
                            .as_ref() as &str,
                    ) {
                        return false;
                    }
                }
                "winner" => {
                    if game.winner != args.single_quoted::<bool>().unwrap() {
                        return false;
                    }
                }
                _ => (),
            }
        }
        return true;
    });

    let mut response = MessageBuilder::new();
    let mut n: usize = 0;
    for game in games {
        response.push(game.to_string() + "\n");
        n += 1;
    }
    response.push(format!("Matches {} games\n", n));

    message.channel_id.say(&context.http, &response)?;
    Ok(())
}
