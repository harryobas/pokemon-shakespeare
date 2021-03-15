use scraper::{Html, Selector};
use reqwest::blocking::{get, Client};
use std::collections::HashMap;
use std::error::Error;

extern crate serde;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
 pub struct TranslatedPokemon{
     pub contents: Contents

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
 pub struct Contents{
    pub translated: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Pokemon{
    pub name: String,
    pub description: String
}

pub fn get_pokemon_description(pokemon_name: &str) 
-> Result<String, Box<dyn Error>> {

    const URL: &str = "https://www.pokemon.com/us/pokedex/";

    let uri = format!("{}{}", URL, pokemon_name);
    let mut pokemon_description = String::new();

    let body = get(&uri)?.text()?;
    
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("p.version-x").expect("parse error");

    for element in fragment.select(&selector) {
        let mut elem_text = element.text().collect::<Vec<_>>();

        if let Some(text) = elem_text.pop() {
            pokemon_description.push_str(text.trim())  
        }

    }

    Ok(pokemon_description)

}

pub fn shakespeare_translate(pokemon_description: &str) 
-> Result<TranslatedPokemon, Box<dyn Error>>{

    const URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";

    let mut params = HashMap::new();

    params.insert("text", pokemon_description);

    let client = Client::new();

    let resp = client.post(URL).form(&params)
        .send()?.text()?;

    let translated_pokemon: TranslatedPokemon = serde_json::from_str(&resp).expect("faild to parse json");

    Ok(translated_pokemon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_get_pokemon_description(){
        let pokemon_name = "Ursaring"; 
        assert!(get_pokemon_description(pokemon_name).is_ok())
    }

    #[test]
    fn test_shakespeare_translate(){
        let txt = "In the forests inhabited by Ursaring, it is said that there are many streams and towering trees where they gather food. This Pokémon walks through its forest gathering food every day.";

        assert!(shakespeare_translate(txt).is_ok())
    }

}
