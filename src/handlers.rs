
use crate::pokemon::{get_pokemon_description, 
    shakespeare_translate, Pokemon};

use rocket::http::Status;
use rocket_contrib::json::Json;

#[rocket::get("/<pokemon>")]
pub fn process(pokemon: String) -> Result<Json<Pokemon>, Status>{
    match get_pokemon_description(&pokemon){
        Err(_e) => Err(Status::NotFound),
        Ok(description) => Ok(Json(translate(&pokemon, &description)))
    }
}

fn translate(pokemon: &str, description: &str) -> Pokemon{
    let t_pokemon = shakespeare_translate(description);
    let pokemon_description = Pokemon{name: pokemon.into(), 
        description: t_pokemon.contents.translated};

    pokemon_description
}

#[cfg(test)]
mod test{
    use super::*;
    use rocket::routes;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn process(){
        let rocket_server = rocket::ignite().mount("/", routes![process]);
        let client = Client::new(rocket_server).expect("valid rocket instance");
        let response = client.get("/charizard").dispatch();

        assert_eq!(response.status(), Status::Ok);
    
    }



}





