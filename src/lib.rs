pub mod formatter;

pub use formatter::format_gdscript;

pub struct FormatterConfig {
    pub indent_size: usize,
    pub use_tabs: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_size: 4,
            use_tabs: false,
        }
    }
}