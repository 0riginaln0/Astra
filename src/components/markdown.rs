use mlua::FromLua;

// Define a Lua-compatible struct to receive options
#[derive(Debug, Clone)]
pub struct LuaCompileOptions {
    pub allow_any_img_src: Option<bool>,
    pub allow_dangerous_html: Option<bool>,
    pub allow_dangerous_protocol: Option<bool>,
    pub default_line_ending: Option<String>,
    pub gfm_footnote_back_label: Option<String>,
    pub gfm_footnote_clobber_prefix: Option<String>,
    pub gfm_footnote_label_attributes: Option<String>,
    pub gfm_footnote_label_tag_name: Option<String>,
    pub gfm_footnote_label: Option<String>,
    pub gfm_task_list_item_checkable: Option<bool>,
    pub gfm_tagfilter: Option<bool>,
}

impl LuaCompileOptions {
    // Helper method to convert to markdown::CompileOptions
    pub fn to_compile_options(&self) -> markdown::CompileOptions {
        markdown::CompileOptions {
            allow_any_img_src: self.allow_any_img_src.unwrap_or(false),
            allow_dangerous_html: self.allow_dangerous_html.unwrap_or(false),
            allow_dangerous_protocol: self.allow_dangerous_protocol.unwrap_or(false),
            default_line_ending: self.default_line_ending.as_ref()
                .map(|s| match s.as_str() {
                    "crlf" => markdown::LineEnding::CarriageReturnLineFeed,
                    "lf" => markdown::LineEnding::LineFeed,
                    _ => markdown::LineEnding::CarriageReturnLineFeed,
                })
                .unwrap_or(markdown::LineEnding::CarriageReturnLineFeed),
            gfm_footnote_back_label: self.gfm_footnote_back_label.clone(),
            gfm_footnote_clobber_prefix: self.gfm_footnote_clobber_prefix.clone(),
            gfm_footnote_label_attributes: self.gfm_footnote_label_attributes.clone(),
            gfm_footnote_label_tag_name: self.gfm_footnote_label_tag_name.clone(),
            gfm_footnote_label: self.gfm_footnote_label.clone(),
            gfm_task_list_item_checkable: self.gfm_task_list_item_checkable.unwrap_or(true),
            gfm_tagfilter: self.gfm_tagfilter.unwrap_or(true),
        }
    }
}

impl FromLua for LuaCompileOptions {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Table(table) => {
                let mut opts = LuaCompileOptions {
                    allow_any_img_src: None,
                    allow_dangerous_html: None,
                    allow_dangerous_protocol: None,
                    default_line_ending: None,
                    gfm_footnote_back_label: None,
                    gfm_footnote_clobber_prefix: None,
                    gfm_footnote_label_attributes: None,
                    gfm_footnote_label_tag_name: None,
                    gfm_footnote_label: None,
                    gfm_task_list_item_checkable: None,
                    gfm_tagfilter: None,
                };

                // Read each possible field from the Lua table
                if let Ok(value) = table.get("allow_any_img_src") {
                    opts.allow_any_img_src = Some(value);
                }
                if let Ok(value) = table.get("allow_dangerous_html") {
                    opts.allow_dangerous_html = Some(value);
                }
                if let Ok(value) = table.get("allow_dangerous_protocol") {
                    opts.allow_dangerous_protocol = Some(value);
                }
                if let Ok(value) = table.get("default_line_ending") {
                    opts.default_line_ending = Some(value);
                }
                if let Ok(value) = table.get("gfm_footnote_back_label") {
                    opts.gfm_footnote_back_label = Some(value);
                }
                if let Ok(value) = table.get("gfm_footnote_clobber_prefix") {
                    opts.gfm_footnote_clobber_prefix = Some(value);
                }
                if let Ok(value) = table.get("gfm_footnote_label_attributes") {
                    opts.gfm_footnote_label_attributes = Some(value);
                }
                if let Ok(value) = table.get("gfm_footnote_label_tag_name") {
                    opts.gfm_footnote_label_tag_name = Some(value);
                }
                if let Ok(value) = table.get("gfm_footnote_label") {
                    opts.gfm_footnote_label = Some(value);
                }
                if let Ok(value) = table.get("gfm_task_list_item_checkable") {
                    opts.gfm_task_list_item_checkable = Some(value);
                }
                if let Ok(value) = table.get("gfm_tagfilter") {
                    opts.gfm_tagfilter = Some(value);
                }

                Ok(opts)
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LuaCompileOptions".to_owned(),
                message: Some("Expected table for options".to_string()),
            }),
        }
    }
}

pub struct MarkdownExtension;

impl MarkdownExtension {
    pub fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        let markdown_table = lua.create_table()?;

        markdown_table.set(
            "to_html",
            lua.create_function(|_lua, (markdown, opts): (String, Option<LuaCompileOptions>)| {
                let compile_options = if let Some(opts) = opts {
                    opts.to_compile_options()
                } else {
                    // Use default GFM options
                    markdown::Options::gfm().compile
                };

                let options = markdown::Options {
                    compile: compile_options,
                    parse: markdown::ParseOptions::gfm(), // You might want to make this configurable too
                };

                let html: Result<String, markdown::message::Message> =
                    markdown::to_html_with_options(&markdown, &options);
                
                match html {
                    Ok(s) => Ok(s),
                    Err(e) => Ok(e.reason.to_string()),
                }
            })?,
        )?;

        // Add a helper function that returns default GFM options as a table
        markdown_table.set(
            "gfm_options",
            lua.create_function(|lua, ()| {
                let table = lua.create_table()?;
                // Set default GFM values
                table.set("allow_any_img_src", false)?;
                table.set("allow_dangerous_html", false)?;
                table.set("allow_dangerous_protocol", false)?;
                table.set("default_line_ending", "crlf")?;
                table.set("gfm_task_list_item_checkable", true)?;
                table.set("gfm_tagfilter", true)?;
                Ok(table)
            })?,
        )?;

        lua.globals().set("markdown", markdown_table)?;

        Ok(())
    }
}