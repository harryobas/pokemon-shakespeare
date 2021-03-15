
use crate::pokemon::{get_pokemon_description, 
    shakespeare_translate, Pokemon};

use rocket::http::Status;
use rocket_contrib::json::Json;

#[rocket::get("/pokemon/<pokemon_name>")]
pub fn process(pokemon_name: String) -> Result<Json<Pokemon>, Status>{
    match get_pokemon_description(&pokemon_name){
        Err(_e) => Err(Status::InternalServerError),
        Ok(description) => {
            match description.is_empty(){
                true => Err(Status::NotFound),
                false => Ok(Json(translate(&pokemon_name, &description)))
            }
        }
    }
}

fn translate(pokemon: &str, description: &str) -> Pokemon{
    let mut pokemon_description: Pokemon = Default::default();

    if let Ok(t_pokemon) = shakespeare_translate(description) {
        pokemon_description.name = pokemon.into();
        pokemon_description.description = t_pokemon.contents.translated;    
    }

    pokemon_description
}

#[cfg(test)]
mod test{
    use super::*;
    use rocket::routes;
    use rocket::local::Client;
    use rocket::http::ContentType;

    #[test]
    fn test_process_with_valid_pokemon_name(){
        let rocket_server = rocket::ignite().mount("/", routes![process]);
        let client = Client::new(rocket_server).expect("valid rocket instance");
        let response = client.get("/pokemon/charizard").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }

    #[test]
    fn test_process_with_invalid_pokemon_name(){
        let rocket_server = rocket::ignite().mount("/", routes![process]);
        let client = Client::new(rocket_server).expect("valid rocket instance");
        let response = client.get("/pokemon/perry").dispatch();

        assert_eq!(response.status(), Status::NotFound);

    }

}





