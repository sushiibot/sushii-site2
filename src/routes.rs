use rocket::State;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::assets::ASSETS;
use crate::model::{commands::CommandList, context::TemplateContext, config::Config};

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        &TemplateContext {
            title: "Home",
            name: None,
            commands: None,
            assets: &ASSETS,
            parent: "layout",
        },
    )
}

#[get("/about")]
pub fn about() -> Template {
    Template::render(
        "about",
        &TemplateContext {
            title: "About",
            name: None,
            commands: None,
            assets: &ASSETS,
            parent: "layout",
        },
    )
}

#[get("/commands")]
pub fn commands(cmds: State<CommandList>) -> Template {
    Template::render(
        "commands",
        &TemplateContext {
            title: "Commands",
            name: None,
            commands: Some(&cmds),
            assets: &ASSETS,
            parent: "layout",
        },
    )
}

#[get("/help")]
pub fn help() -> Template {
    Template::render(
        "help",
        &TemplateContext {
            title: "Help",
            name: None,
            commands: None,
            assets: &ASSETS,
            parent: "layout",
        },
    )
}

#[get("/hello/<name>")]
pub fn hello(name: String) -> Template {
    Template::render(
        "hello",
        &TemplateContext {
            title: "Hello",
            name: Some(name),
            commands: None,
            assets: &ASSETS,
            parent: "layout",
        },
    )
}

#[get("/invite")]
pub fn invite(config: State<'_, Config>) -> Redirect {
    Redirect::to(config.invite_url.clone())
}
