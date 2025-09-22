//! This module formats GDScript code using Topiary with tree-sitter to parse and
//! format GDScript files.
//!
//! After the main formatting pass through Topiary, we apply post-processing steps
//! to clean up and standardize the output. These include:
//!
//! - Adding vertical spacing between methods, classes, etc.
//! - Removing unnecessary blank lines that might have been added during formatting
//! - Removing dangling semicolons that sometimes end up on their own lines
//! - Cleaning up lines that contain only whitespace
//! - Optionally reordering code elements according to the GDScript style guide
//!
//! Some of the post-processing is outside of Topiary's capabilities, while other
//! rules have too much performance overhead when applied through Topiary.
use std::io::BufWriter;

use regex::RegexBuilder;
use topiary_core::{formatter, Language, Operation, TopiaryQuery};
use tree_sitter::{Query, QueryCursor, StreamingIterator, Tree};

use crate::FormatterConfig;

static QUERY: &str = include_str!("../queries/gdscript.scm");

pub fn format_gdscript(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    format_gdscript_with_config(content, &FormatterConfig::default())
}

pub fn format_gdscript_with_config(
    content: &str,
    config: &FormatterConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut formatter = Formatter::new(content.to_owned(), config.clone());

    formatter.preprocess().format()?.postprocess().reorder();
    formatter.finish()
}

struct Formatter {
    content: String,
    config: FormatterConfig,
    input_tree: Option<Tree>,
}

impl Formatter {
    #[inline(always)]
    fn new(content: String, config: FormatterConfig) -> Self {
        // Save original syntax tree for verification
        let input_tree = if config.safe {
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&tree_sitter_gdscript::LANGUAGE.into())
                .unwrap();
            Some(parser.parse(&content, None).unwrap())
        } else {
            None
        };

        Self {
            content,
            config,
            input_tree,
        }
    }

    #[inline(always)]
    fn format(&mut self) -> Result<&mut Self, Box<dyn std::error::Error>> {
        let indent_string = if self.config.use_spaces {
            " ".repeat(self.config.indent_size)
        } else {
            "\t".to_string()
        };

        let language = Language {
            name: "gdscript".to_owned(),
            query: TopiaryQuery::new(&tree_sitter_gdscript::LANGUAGE.into(), QUERY).unwrap(),
            grammar: tree_sitter_gdscript::LANGUAGE.into(),
            indent: Some(indent_string),
        };

        let mut input = self.content.as_bytes();
        let mut output = Vec::new();
        let mut writer = BufWriter::new(&mut output);

        formatter(
            &mut input,
            &mut writer,
            &language,
            Operation::Format {
                skip_idempotence: true,
                tolerate_parsing_errors: true,
            },
        )
        .map_err(|e| format!("Topiary formatting failed: {e}"))?;

        drop(writer);

        self.content = String::from_utf8(output)
            .map_err(|e| format!("Failed to parse topiary output as UTF-8: {}", e))?;

        Ok(self)
    }

    #[inline(always)]
    fn reorder(&mut self) -> &mut Self {
        if !self.config.reorder_code {
            return self;
        }
        match crate::reorder::reorder_gdscript_elements(&self.content) {
            Ok(reordered) => {
                self.content = reordered;
            }
            Err(e) => {
                eprintln!(
                    "Warning: Code reordering failed: {e}. Returning formatted code without reordering."
                );
            }
        };
        self
    }

    /// This function runs over the content before going through topiary.
    /// It is used to prepare the content for formatting or save performance by
    /// pre-applying rules that could be performance-intensive through topiary.
    #[inline(always)]
    fn preprocess(&mut self) -> &mut Self {
        self.remove_newlines_after_extends_statement()
    }

    /// This function runs over the content after going through topiary. We use it
    /// to clean up/balance out the output.
    #[inline(always)]
    fn postprocess(&mut self) -> &mut Self {
        self.clean_up_lines_with_only_whitespace()
            .fix_dangling_semicolons()
            .postprocess_tree_sitter()
    }

    /// Finishes formatting and returns the resulting file content.
    #[inline(always)]
    fn finish(self) -> Result<String, Box<dyn std::error::Error>> {
        // This will be Some if config.safe is true
        if let Some(input_tree) = self.input_tree {
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&tree_sitter_gdscript::LANGUAGE.into())
                .unwrap();
            let tree = parser.parse(&self.content, None).unwrap();

            if !compare_trees(input_tree, tree) {
                return Err("Trees are different".into());
            }
        }

        Ok(self.content)
    }

    /// This function removes additional new line characters after `extends_statement`.
    #[inline(always)]
    fn remove_newlines_after_extends_statement(&mut self) -> &mut Self {
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
        self.content = re
            .replace(&self.content, "$extends_line$extends_name\n")
            .to_string();
        self
    }

    /// This function cleans up lines that contain only whitespace characters
    /// (spaces, tabs) and a newline character. It only keeps a single newline
    /// character.
    #[inline(always)]
    fn clean_up_lines_with_only_whitespace(&mut self) -> &mut Self {
        let re = RegexBuilder::new(r"^\s+\n$")
            .multi_line(true)
            .build()
            .expect("empty line regex should compile");
        self.content = re.replace_all(&self.content, "\n").to_string();
        self
    }

    /// This function fixes semicolons that end up on their own line with indentation
    /// by moving them to the end of the previous line.
    #[inline(always)]
    fn fix_dangling_semicolons(&mut self) -> &mut Self {
        if !self.content.contains(";") {
            return self;
        }
        let re_trailing = RegexBuilder::new(r"(\s*;)+$")
            .multi_line(true)
            .build()
            .expect("semicolon regex should compile");
        self.content = re_trailing.replace_all(&self.content, "").to_string();
        self
    }

    /// This function runs postprocess passes that uses tree-sitter.
    #[inline(always)]
    fn postprocess_tree_sitter(&mut self) -> &mut Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_gdscript::LANGUAGE.into())
            .unwrap();
        let mut tree = parser.parse(&self.content, None).unwrap();

        Self::handle_two_blank_line(&mut tree, &mut self.content);

        self
    }

    /// This function makes sure we have the correct vertical spacing between important definitions:
    /// Two blank lines between function definitions, inner classes, etc. Taking any
    /// comments or docstrings into account.
    ///
    /// This uses tree-sitter to find the relevant nodes and their positions.
    fn handle_two_blank_line(tree: &mut Tree, content: &mut String) {
        let root = tree.root_node();
        let sibling_definition_query = match Query::new(
            &tree_sitter::Language::new(tree_sitter_gdscript::LANGUAGE),
            "(([(variable_statement) (function_definition) (class_definition) (signal_statement) (const_statement) (enum_definition) (constructor_definition)]) @first
    . (([(comment) (annotation)])* @comment . ([(function_definition) (constructor_definition) (class_definition)]) @second))",
        ) {
            Ok(q) => q,
            Err(err) => {
                panic!("Failed to create query: {}", err);
            }
        };

        // First we need to find all the places where we should add blank lines.
        // We can't modify the content string while tree-sitter is borrowing it, so we
        // collect all the positions first, then make changes afterward.
        let mut new_lines_at = Vec::new();
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&sibling_definition_query, root, content.as_bytes());
        while let Some(m) = matches.next() {
            let first_node = m.captures[0].node;
            if m.captures.len() == 3 {
                let comment_node = m.captures[1].node;
                let second_node = m.captures[2].node;
                // If the @comment is on the same line as the first node,
                // we'll add a blank line before the @second node
                if comment_node.start_position().row == first_node.start_position().row {
                    // Find where to insert the new line (before any indentation)
                    let mut byte_idx = second_node.start_byte();
                    let mut position = second_node.start_position();
                    position.column = 0;
                    while content.as_bytes()[byte_idx] != b'\n' {
                        byte_idx -= 1;
                    }
                    new_lines_at.push((byte_idx, position));
                } else {
                    // Otherwise, add a blank line after the first node
                    new_lines_at.push((first_node.end_byte(), first_node.end_position()));
                }
            } else {
                // If there's no comment between the nodes, add a blank line after the first node
                new_lines_at.push((first_node.end_byte(), first_node.end_position()));
            }
        }

        // We sort the positions in reverse order so that when we insert new lines,
        // we don't mess up the positions of the other insertions we need to make.
        new_lines_at.sort_by(|a, b| b.cmp(a));

        for (byte_idx, position) in new_lines_at {
            let mut new_end_position = position;
            let mut new_end_byte_idx = byte_idx;
            // Only add a second blank line if there isn't already one
            if content.as_bytes()[byte_idx + 1] != b'\n' {
                new_end_position.row += 1;
                new_end_byte_idx += 1;
                content.insert(byte_idx, '\n');
            }
            // Add the first blank line
            new_end_position.row += 1;
            new_end_byte_idx += 1;
            content.insert(byte_idx, '\n');

            // Update the tree sitter parse tree to reflect our changes so that any
            // future processing will work with the correct positions
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
}

/// Returns true if both trees have the same structure.
fn compare_trees(left_tree: Tree, right_tree: Tree) -> bool {
    let mut left_cursor = left_tree.walk();
    let mut right_cursor = right_tree.walk();

    let mut left_stack = Vec::new();
    let mut right_stack = Vec::new();
    left_stack.push(left_cursor.node());
    right_stack.push(right_cursor.node());

    while let (Some(left_current_node), Some(right_current_node)) =
        (left_stack.pop(), right_stack.pop())
    {
        if left_current_node.child_count() != right_current_node.child_count() {
            // A different number of children means the syntax trees are different, so the code
            // structure has changed.
            // NOTE: There's a valid case of change: an annotation above a variable may be wrapped
            // on the same line as the variable, which turns the annotation into a child of the variable.
            // We could ignore this specific case, but for now, we consider any change in structure
            // as a potential issue.
            return false;
        }

        let left_children = left_current_node.children(&mut left_cursor);
        let right_children = left_current_node.children(&mut right_cursor);
        for (left_node, right_node) in left_children.zip(right_children) {
            if left_node.grammar_id() != right_node.grammar_id() {
                return false;
            }
            left_stack.push(left_node);
            right_stack.push(right_node);
        }
    }
    true
}
