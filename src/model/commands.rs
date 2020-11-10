use serde::{Deserialize, Deserializer, Serialize};
use pulldown_cmark::{html, Parser};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Permission {
    pub name: String,
    pub color: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    /// Regular message
    #[serde(default)]
    #[serde(deserialize_with = "to_markdown_html")]
    pub content: Option<String>,
    /// Command content without prefix, prefix is added later in template
    pub command: Option<String>,

    #[serde(default)]
    pub bot: bool,
}

fn to_markdown_html<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;

    Ok(s.map(|s| {
        let parser = Parser::new(&s);

        let mut html_buf = String::new();
        html::push_html(&mut html_buf, parser);

        html_buf
    }))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Command {
    pub name: String,
    pub aliases: Option<Vec<String>>,
    pub description: String,
    pub required_permissions: Option<Vec<Permission>>,
    pub usage: Option<String>,
    pub examples: Option<Vec<Message>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandGroup {
    pub name: String,
    pub description: Option<String>,
    pub commands: Vec<Command>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandList {
    pub groups: Vec<CommandGroup>,
}
