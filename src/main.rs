#[macro_use]
extern crate rocket;

mod assets;
mod catchers;
mod model;
mod routes;

#[cfg(test)]
mod tests;

use figment::{
    providers::{Format, Json, Toml, Yaml},
    Figment,
};
use rocket_contrib::{
    serve::StaticFiles,
    templates::Template,
};

use crate::model::commands::CommandList;
use crate::routes::*;

#[launch]
fn rocket() -> rocket::Rocket {
    let cmds: CommandList = Figment::new()
        .merge(Toml::file("Commands.toml"))
        .merge(Json::file("Commands.json"))
        .merge(Yaml::file("Commands.yml"))
        .extract()
        .expect("Missing commands list");

    rocket::ignite()
        .manage(cmds)
        .mount("/static", StaticFiles::from("./static"))
        .mount("/", routes![index, commands, about, help, hello])
        .register(catchers![catchers::not_found])
        .attach(Template::fairing())
}
