use scraper::{Html, Selector};
use reqwest::blocking::{get, Client};
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
 pub struct TranslatedPokemon{
     //success: Success,
     contents: Contents

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Success{
    total: i32
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
 struct Contents{
    translated: String,
    //text: String
}

pub fn get_pokemon_description(pokemon_name: &str) -> String {
    const URL: &str = "https://www.pokemon.com/us/pokedex/";

    let uri = format!("{}{}", URL, pokemon_name);
    let mut pokemon_description = String::new();

    let body = get(&uri)
        .expect("not found")
        .text()
        .expect("decode error");
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

pub fn shakespeare_translate(pokemon_description: &str) -> TranslatedPokemon{

    const URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";

    let mut params = HashMap::new();
    params.insert("text", pokemon_description);

    let client = Client::new();

    let resp = client.post(URL).form(&params)
        .send()
        .expect("faild to get json response")
        .text()
        .expect("faild to decode");

    let translated_pokemon: TranslatedPokemon = serde_json::from_str(&resp).expect("faild to parse json");

    translated_pokemon
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

    #[test]
    fn test_shakespeare_translate(){
        let translate = "In the forests did inhabit by ursaring,  't is did doth sayeth yond thither art many streams and towering trees whither they gather food. This pokémon walks through its forest gathering food every day.";
        let txt =  "In the forests inhabited by Ursaring, it is said that there are many streams and towering trees where they gather food. This Pokémon walks through its forest gathering food every day.";

        let contents = Contents{translated: translate.to_string()};
        let t_pokemon = TranslatedPokemon{contents: contents};

        assert_eq!(shakespeare_translate(txt), t_pokemon)
    }

}
