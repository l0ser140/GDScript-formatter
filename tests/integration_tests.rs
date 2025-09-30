use gdscript_formatter::FormatterConfig;
use gdscript_formatter::formatter::format_gdscript_with_config;
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::path::Path;

test_each_file::test_each_path! { in "./tests/input" => test_file }
test_each_file::test_each_path! { in "./tests/reorder_code/input" => test_reorder_file }

fn make_whitespace_visible(s: &str) -> String {
    s.replace(' ', "·")
        .replace('\t', "⇥   ")
        .replace('\n', "↲\n")
}

fn assert_formatted_eq(
    result: &str,
    expected: &str,
    file_path: &Path,
    error_context_message: &str,
) {
    if result != expected {
        eprintln!("\n{} - {}", error_context_message, file_path.display());
        eprintln!("Diff between expected(-) and actual output(+):");
        let diff = TextDiff::from_lines(expected, result);
        for change in diff.iter_all_changes() {
            let text = make_whitespace_visible(&change.to_string());
            match change.tag() {
                ChangeTag::Insert => eprint!("\x1B[92m+{}\x1B[0m", text),
                ChangeTag::Delete => eprint!("\x1B[91m-{}\x1B[0m", text),
                ChangeTag::Equal => eprint!(" {}", text),
            }
        }
        eprintln!("\nRaw strings:");
        eprintln!("\nEXPECTED (raw):");
        eprintln!("{:?}", expected);
        eprintln!("\nGOT (raw):");
        eprintln!("{:?}", result);
        panic!("Assertion failed: {}", error_context_message);
    }
}

fn test_file(file_path: &Path) {
    test_file_with_config(file_path, &FormatterConfig::default(), true);
}

fn test_reorder_file(file_path: &Path) {
    test_file_with_config(
        file_path,
        &FormatterConfig {
            reorder_code: true,
            ..Default::default()
        },
        true,
    );
}

fn test_file_with_config(file_path: &Path, config: &FormatterConfig, check_idempotence: bool) {
    let file_name = file_path.file_name().expect("path is not a file path");

    let input_path = file_path;
    let expected_path = file_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("expected/")
        .join(file_name);

    let input_content =
        fs::read_to_string(&input_path).expect(&format!("Failed to read {}", input_path.display()));
    let expected_content = fs::read_to_string(&expected_path)
        .expect(&format!("Failed to read {}", expected_path.display()));

    let result = format_gdscript_with_config(&input_content, config)
        .expect(&format!("Failed to format {}", input_path.display()));

    assert_formatted_eq(
        &result,
        &expected_content,
        &input_path,
        "First formatting output doesn't match expected",
    );

    if check_idempotence {
        let second_result = format_gdscript_with_config(&result, config)
            .expect(&format!("Failed to format {}", input_path.display()));
        assert_formatted_eq(
            &second_result,
            &result,
            &input_path,
            "Idempotence check failed, formatting a second time gave different results",
        );
    }
}
