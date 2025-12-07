---@meta

---@class MarkdownOptions
---Compilation options for markdown conversion
---@field allow_any_img_src? boolean            Allow any image source (default: false)
---@field allow_dangerous_html? boolean         Allow dangerous HTML (default: false)
---@field allow_dangerous_protocol? boolean     Allow dangerous protocols (default: false)
---@field default_line_ending? string           Line ending type: "crlf" or "lf" (default: "crlf")
---@field gfm_footnote_back_label? string       Footnote back label (default: nil)
---@field gfm_footnote_clobber_prefix? string   Footnote clobber prefix (default: nil)
---@field gfm_footnote_label_attributes? string Footnote label attributes (default: nil)
---@field gfm_footnote_label_tag_name? string   Footnote label tag name (default: nil)
---@field gfm_footnote_label? string            Footnote label (default: nil)
---@field gfm_task_list_item_checkable? boolean Make task list items checkable (default: true)
---@field gfm_tagfilter? boolean                Enable GFM tagfilter (default: true)

---@class Markdown
---Markdown processing utilities
local Markdown = {}

---Converts markdown text to HTML
---@param markdown string           # Markdown text to convert
---@param opts? MarkdownOptions|nil # Optional compilation options table
---@return string html              # Converted HTML
function Markdown.to_html(markdown, opts) end

---Creates a table with default GFM (GitHub Flavored Markdown) options
---Can be modified and passed to `to_html` for customization
---@return MarkdownOptions opts     # Default GFM options table
function Markdown.gfm_options() end

---@class Markdown
markdown = {}

return markdown