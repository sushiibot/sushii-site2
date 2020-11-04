use serde::Serialize;
use super::commands::CommandList;

#[derive(Serialize)]
pub struct TemplateContext<'a> {
    pub title: &'static str,
    pub name: Option<String>,
    pub commands: Option<&'a CommandList>,
    // This key tells handlebars which template is the parent.
    pub parent: &'static str,
}
