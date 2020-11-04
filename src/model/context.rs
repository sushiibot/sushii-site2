use serde::Serialize;
use super::commands::CommandList;

#[derive(Serialize)]
pub struct TemplateContext<'a> {
    pub title: &'static str,
    pub name: Option<String>,
    pub commands: Option<&'a CommandList>,
    pub assets: &'static AssetFiles,
    // This key tells handlebars which template is the parent.
    pub parent: &'static str,
}

#[derive(Clone, Serialize)]
pub struct CSSFiles {
    pub app: String,
    pub fonts: String,
    pub vendor: String,
}
#[derive(Clone, Serialize)]
pub struct JSFiles {
    pub app: String,
}
#[derive(Clone, Serialize)]
pub struct AssetFiles {
    pub css: CSSFiles,
    pub js: JSFiles,
}
