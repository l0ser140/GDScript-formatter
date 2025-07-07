use gdscript_formatter::format_gdscript;
use std::fs;
use std::path::Path;

#[test]
fn test_all_sample_files() {
    let input_dir = Path::new("tests/input");
    let expected_dir = Path::new("tests/expected");

    if !input_dir.exists() || !expected_dir.exists() {
        panic!("Test directories do not exist");
    }

    let input_files = fs::read_dir(input_dir)
        .expect("Failed to read input directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "gd"))
        .collect::<Vec<_>>();

    for input_file in input_files {
        let input_path = input_file.path();
        let expected_path = expected_dir.join(input_file.file_name());

        if !expected_path.exists() {
            panic!("Expected file not found: {}", expected_path.display());
        }

        let input_content = fs::read_to_string(&input_path)
            .expect(&format!("Failed to read {}", input_path.display()));
        let expected_content = fs::read_to_string(&expected_path)
            .expect(&format!("Failed to read {}", expected_path.display()));

        let result = format_gdscript(&input_content)
            .expect(&format!("Failed to format {}", input_path.display()));

        assert_eq!(
            result,
            expected_content,
            "Formatted output doesn't match expected for {}",
            input_path.display()
        );
    }
}
