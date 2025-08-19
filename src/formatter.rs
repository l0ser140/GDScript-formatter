use std::io::BufWriter;

use topiary_core::{formatter, Language, Operation, TopiaryQuery};

use crate::FormatterConfig;

pub fn format_gdscript(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    format_gdscript_with_config(content, &FormatterConfig::default())
}

pub fn format_gdscript_with_config(
    content: &str,
    config: &FormatterConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = include_str!("../queries/gdscript.scm");

    let indent_string = if config.use_tabs {
        "\t".to_string()
    } else {
        " ".repeat(config.indent_size)
    };

    let language: Language = Language {
        name: "gdscript".to_owned(),
        query: TopiaryQuery::new(&tree_sitter_gdscript::LANGUAGE.into(), query).unwrap(),
        grammar: tree_sitter_gdscript::LANGUAGE.into(),
        indent: Some(indent_string),
    };

    let mut input = content.as_bytes();
    let mut output = BufWriter::new(Vec::new());

    let formatter_result = formatter(
        &mut input,
        &mut output,
        &language,
        Operation::Format {
            skip_idempotence: true,
            tolerate_parsing_errors: true,
        },
    );

    if let Err(formatter_error) = formatter_result {
        return Err(format!("Topiary formatting failed: {}", formatter_error).into());
    }

    // TODO: is it possible to remove additional heap allocation here?
    let formatted_content = String::from_utf8(output.buffer().to_vec())
        .map_err(|e| format!("Failed to parse topiary output as UTF-8: {}", e))?;

    Ok(formatted_content)
}
