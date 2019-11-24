extern crate proc_macro;

use proc_macro::TokenStream;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Author {
    name: String,
    age: u8,
    city: Option<String>,
}

#[derive(Deserialize, Debug)]
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

#[proc_macro]
pub fn deserialize_json(_item: TokenStream) -> TokenStream {
    let games: Vec<Game> = serde_json::from_slice(include_bytes!("data/games.json")).unwrap();

    format!("{:#?}", games)
        .replace("[", "std::vec![")
        .parse()
        .unwrap()
}
