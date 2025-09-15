use std::io::BufWriter;

use regex::RegexBuilder;
use topiary_core::{formatter, Language, Operation, TopiaryQuery};
use tree_sitter::{Query, QueryCursor, StreamingIterator, Tree};

use crate::FormatterConfig;

pub fn format_gdscript(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    format_gdscript_with_config(content, &FormatterConfig::default())
}

pub fn format_gdscript_with_config(
    content: &str,
    config: &FormatterConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = include_str!("../queries/gdscript.scm");

    let indent_string = if config.use_spaces {
        " ".repeat(config.indent_size)
    } else {
        "\t".to_string()
    };

    let language: Language = Language {
        name: "gdscript".to_owned(),
        query: TopiaryQuery::new(&tree_sitter_gdscript::LANGUAGE.into(), query).unwrap(),
        grammar: tree_sitter_gdscript::LANGUAGE.into(),
        indent: Some(indent_string),
    };

    let preprocessed_content = preprocess(content);

    let mut input = preprocessed_content.as_bytes();
    let mut output = Vec::new();
    let mut writer = BufWriter::new(&mut output);

    let formatter_result = formatter(
        &mut input,
        &mut writer,
        &language,
        Operation::Format {
            skip_idempotence: true,
            tolerate_parsing_errors: true,
        },
    );

    if let Err(formatter_error) = formatter_result {
        return Err(format!("Topiary formatting failed: {}", formatter_error).into());
    }

    drop(writer);

    let mut formatted_content = String::from_utf8(output)
        .map_err(|e| format!("Failed to parse topiary output as UTF-8: {}", e))?;

    formatted_content = postprocess(formatted_content);
    formatted_content = postprocess_tree_sitter(formatted_content);

    Ok(formatted_content)
}

/// This function runs postprocess passes that uses tree-sitter.
fn postprocess_tree_sitter(mut content: String) -> String {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_gdscript::LANGUAGE.into())
        .unwrap();
    let mut tree = parser.parse(&content, None).unwrap();

    handle_two_blank_line(&mut tree, &mut content);

    content
}

/// This function ensures that some statements has two blank lines between them.
fn handle_two_blank_line(tree: &mut Tree, content: &mut String) {
    let root = tree.root_node();
    let q = match Query::new(
        &tree_sitter::Language::new(tree_sitter_gdscript::LANGUAGE),
        "(([(variable_statement) (function_definition) (class_definition) (signal_statement) (const_statement) (enum_definition) (constructor_definition)]) @first
. ((comment)* @comment . ([(function_definition) (constructor_definition) (class_definition)]) @second))",
    ) {
        Ok(q) => q,
        Err(err) => {
            panic!("{}", err);
        }
    };

    // collect positions for the new lines first because we can't
    // modify string while it is borrowed by tree-sitter
    let mut new_lines_at = Vec::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&q, root, content.as_bytes());
    while let Some(m) = matches.next() {
        let first_node = m.captures[0].node;
        if m.captures.len() == 3 {
            let comment_node = m.captures[1].node;
            let second_node = m.captures[2].node;
            // if @comment node and @first node is on the same line insert new line BEFORE @second node
            if comment_node.start_position().row == first_node.start_position().row {
                // insert new line BEFORE indentation
                let mut byte_idx = second_node.start_byte();
                let mut position = second_node.start_position();
                position.column = 0;
                while content.as_bytes()[byte_idx] != b'\n' {
                    byte_idx -= 1;
                }
                new_lines_at.push((byte_idx, position));
            } else {
                new_lines_at.push((first_node.end_byte(), first_node.end_position()));
            }
        } else {
            // if there is no @comment between two nodes then insert new line AFTER @first node
            new_lines_at.push((first_node.end_byte(), first_node.end_position()));
        }
    }

    // sort in descending order to avoid shifting indices
    // when inserting new lines
    new_lines_at.sort_by(|a, b| b.cmp(a));

    for (byte_idx, position) in new_lines_at {
        let mut new_end_position = position;
        let mut new_end_byte_idx = byte_idx;
        // do not insert second blank line if there is already one
        if content.as_bytes()[byte_idx + 1] != b'\n' {
            new_end_position.row += 1;
            new_end_byte_idx += 1;
            content.insert(byte_idx, '\n');
        }
        new_end_position.row += 1;
        new_end_byte_idx += 1;
        content.insert(byte_idx, '\n');

        // apply edits to tree so that later postprocess passes can still get correct node positions
        tree.edit(&tree_sitter::InputEdit {
            start_byte: byte_idx,
            old_end_byte: byte_idx,
            new_end_byte: new_end_byte_idx,
            start_position: position,
            old_end_position: position,
            new_end_position,
        });
    }
}

/// This function runs over the content before going through topiary.
/// It is used to prepare the content for formatting or save performance by
/// pre-applying rules that could be performance-intensive through topiary.
fn preprocess(content: &str) -> String {
    let mut content = content.to_owned();

    content = remove_newlines_after_extends_statement(content);

    content
}

/// This function runs over the content after going through topiary. We use it
/// to clean up/balance out the output.
fn postprocess(mut content: String) -> String {
    content = clean_up_lines_with_only_whitespace(content);
    if content.contains(';') {
        content = fix_dangling_semicolons(content);
    }

    content
}

/// Remove additional new line characters after `extends_statement`
fn remove_newlines_after_extends_statement(mut content: String) -> String {
    // This regex matches substrings which:
    // - must NOT contain "#" or "\n" characters between new line and "extends" keyword
    // - must end with at least one new line character
    // - must contain `extends_name` character sequence that satisfies one of the following conditions:
    //   - consists out of alphanumeric characters
    //   - consists out of any characters (except new lines) between double quotes
    let re = RegexBuilder::new(
        r#"(?P<extends_line>^[^#\n]*extends )(?P<extends_name>([a-zA-Z0-9]+|".*?"))\n(\n*)"#,
    )
    .multi_line(true)
    .build()
    .expect("regex should compile");
    content = re
        .replace(&content, "$extends_line$extends_name\n")
        .to_string();
    content
}

/// This function cleans up lines that contain only whitespace characters
/// (spaces, tabs) and a newline character. It only keeps a single newline
/// character.
fn clean_up_lines_with_only_whitespace(mut content: String) -> String {
    let re = RegexBuilder::new(r"^\s+\n$")
        .multi_line(true)
        .build()
        .expect("empty line regex should compile");
    content = re.replace_all(&content, "\n").to_string();

    content
}

/// This function fixes semicolons that end up on their own line with indentation
/// by moving them to the end of the previous line.
fn fix_dangling_semicolons(mut content: String) -> String {
    let re_trailing = RegexBuilder::new(r"\s+;$")
        .multi_line(true)
        .build()
        .expect("semicolon regex should compile");
    content = re_trailing.replace_all(&content, "").to_string();

    content
}
