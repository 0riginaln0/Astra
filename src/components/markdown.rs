use mlua::FromLua;

#[derive(Debug, Clone, FromLua)]
pub struct MarkdownExtension;

impl MarkdownExtension {
    pub fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        // Create markdown table
        let markdown_table = lua.create_table()?;

        // Add to_html function
        markdown_table.set(
            "to_html",
            lua.create_function(|_, markdown: String| {
                let html: Result<String, markdown::message::Message> =
                    markdown::to_html_with_options(&markdown, &markdown::Options::gfm());
                match html {
                    Ok(s) => Ok(s),
                    Err(e) => Ok(e.reason.to_string()),
                }
            })?,
        )?;

        // Register table to global
        lua.globals().set("markdown", markdown_table)?;

        Ok(())
    }
}
