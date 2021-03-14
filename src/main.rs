#![feature(decl_macro, proc_macro_hygiene)]

mod pokemon;
mod handlers;
mod route;

fn main() {
    route::create_routes();
}
