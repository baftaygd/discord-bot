use serde::Deserialize;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, StandardFramework,
    },
    model::channel::Message,
    prelude::*,
};

#[derive(Deserialize)]
struct Author {
    name: String,
    age: u8,
    city: String,
}

#[derive(Deserialize)]
struct Game {
    title: String,
    tools: Option<Vec<String>>,
    platforms: Option<Vec<String>>,
    authors: Option<Vec<Author>>,
    year: i32,
    award: String,
    group: String,
    quote: String,
    description: String,
    winner: bool,
}

struct Games;

impl TypeMapKey for Games {
    type Value = Vec<Game>;
}

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
            .configure(|configuration| configuration.prefix("!").delimiters(vec![" "]))
            .group(&DEFAULT_GROUP),
    );
    {
        let file = std::fs::File::open("data/games.json").unwrap();
        let reader = std::io::BufReader::new(file);
        client
            .data
            .write()
            .insert::<Games>(serde_json::from_reader(reader).unwrap());
    }
    client.start().unwrap();
}

#[command]
fn games(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let data = context.data.read();
    let mut games = data.get::<Games>().unwrap().iter().filter(|game| {
        args.restore();
        while !args.is_empty() {
            match args.single_quoted::<String>().unwrap().as_ref() {
                "author" => {
                    if let Some(authors) = &game.authors {
                        let search_author = args.single_quoted::<String>().unwrap().to_lowercase();
                        for (i, author) in authors.iter().enumerate() {
                            if author.name.to_lowercase().contains(search_author.as_ref() as &str) {
                                break;
                            }
                            if i == authors.len() - 1 {
                                return false;
                            }
                        }
                    } else {
                        return false;
                    }
                },
                "title" => {
                    if !game.title.to_lowercase().contains(args.single_quoted::<String>().unwrap().to_lowercase().as_ref() as &str) {
                        return false;
                    }
                },
                "award" => {
                    if !game.award.to_lowercase().contains(args.single_quoted::<String>().unwrap().to_lowercase().as_ref() as &str) {
                        return false;
                    }
                },
                _ => (),
            }
        }
        return true;
    });

    message.channel_id.say(
        &context.http,
        if let Some(game) = games.next() {
            format!("Found game {}!!!", game.title)
        } else {
            "Couldn't find what you wanted :(".to_string()
        },
    )?;
    Ok(())
}
