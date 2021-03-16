use crate::handlers;
use rocket::routes;

pub fn create_routes(){
    rocket::ignite()
        .mount("/", routes![handlers::describe])
        .launch();
}