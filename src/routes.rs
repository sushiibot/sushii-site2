use rocket::{Request, State};
use rocket_contrib::templates::Template;

use crate::model::{template::TemplateContext, commands::CommandList};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", &TemplateContext {
        title: "sushii 2",
        name: None,
        commands: None,
        parent: "layout",
    })
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("about", &TemplateContext {
        title: "About",
        name: None,
        commands: None,
        parent: "layout",
    })
}

#[get("/commands")]
pub fn commands(cmds: State<CommandList>) -> Template {
    Template::render("commands", &TemplateContext {
        title: "Commands",
        name: None,
        commands: Some(&cmds),
        parent: "layout",
    })
}

#[get("/hello/<name>")]
pub fn hello(name: String) -> Template {
    Template::render("hello", &TemplateContext {
        title: "Hello",
        name: Some(name),
        commands: None,
        parent: "layout",
    })
}

