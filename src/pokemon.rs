use std::error::Error;
use scraper::{Html, Selector};
use reqwest::blocking::{self, get, Client};


pub fn get_pokemon_description(pokemon_name: &str) -> String {
    const URL: &str = "https://www.pokemon.com/us/pokedex/";

    let uri = format!("{}{}", URL, pokemon_name);
    let mut pokemon_description = String::new();

    let body = get(&uri).expect("not found")
    .text().expect("decode error");
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("p.version-x").expect("parse error");

    for element in fragment.select(&selector) {
        let mut elem_text = element.text().collect::<Vec<_>>();

        if let Some(text) = elem_text.pop() {
            pokemon_description.push_str(text.trim())  
        }
       
    }

    pokemon_description

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_get_pokemon_description_with_valid_pokemon_name(){
        let pokemon_name = "Ursaring";
        let description = "In the forests inhabited by Ursaring, it is said that there are many streams and towering trees where they gather food. This Pokémon walks through its forest gathering food every day."; 
        assert_eq!(get_pokemon_description(pokemon_name), description)
    }

    #[test]
    fn test_get_pokemon_description_with_invalid_pokemon_name(){
        let pokemon_name = "perry";
        assert_eq!(get_pokemon_description(pokemon_name), "")

    }

}
