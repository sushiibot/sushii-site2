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
use rocket_prometheus::PrometheusMetrics;

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

    let prometheus = PrometheusMetrics::new();

    rocket::ignite()
        .manage(cmds)
        .attach(prometheus.clone())
        .attach(Template::fairing())
        .mount("/metrics", prometheus)
        .mount("/static", StaticFiles::from("./static"))
        .mount("/", routes![index, commands, about, help, hello])
        .register(catchers![catchers::not_found])
}
