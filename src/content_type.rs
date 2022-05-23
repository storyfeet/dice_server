pub const CONTENT_TYPE: &str = "Content-Type";
pub const CT_HTML: &str = "text/html";
pub const CT_JS: &str = "application/javascript";
pub const CT_JSON: &str = "application/json";
pub const CT_CSS: &str = "text/css";
pub const CT_PLAIN: &str = "text/plain";

pub fn ctype_from_path(p: &std::path::Path) -> &'static str {
    match p.extension().and_then(|s| s.to_str()) {
        Some("css") => CT_CSS,
        Some("html") => CT_HTML,
        Some("json") => CT_JSON,
        Some("js") => CT_JS,
        _ => CT_PLAIN,
    }
}
