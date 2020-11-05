#[macro_use] extern crate rocket;

mod assets;
mod catchers;
mod model;
mod routes;

#[cfg(test)] mod tests;

use rocket_contrib::{templates::Template, serve::{StaticFiles, crate_relative}};
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

    rocket::ignite()
        .manage(cmds)
        .mount("/static", StaticFiles::from(crate_relative!("/static")))
        .mount("/", routes![index, commands, about, help, hello])
        .register(catchers![catchers::not_found])
        .attach(Template::fairing())
}
