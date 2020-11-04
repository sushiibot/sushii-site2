#[macro_use] extern crate rocket;

mod catchers;
mod model;
mod routes;

#[cfg(test)] mod tests;

use rocket_contrib::templates::Template;
use figment::{Figment, providers::{Format, Toml, Json, Env}};

use crate::routes::*;
use crate::model::commands::CommandList;

#[launch]
fn rocket() -> rocket::Rocket {
    let cmds: CommandList = Figment::new()
        .merge(Toml::file("Commands.toml"))
        .merge(Json::file("Commands.json"))
        .extract()
        .expect("Missing commands list");
    
    println!("commands: {:#?}", cmds);

    rocket::ignite()
        .manage(cmds)
        .mount("/", routes![index, commands, about, hello])
        .register(catchers![catchers::not_found])
        .attach(Template::fairing())
}
