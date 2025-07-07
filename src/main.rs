use clap::{Arg, Command};
use gdscript_formatter::format_gdscript;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("gdscript-formatter")
        .version("0.1.0")
        .about("A GDScript code formatter using Topiary and Tree-sitter")
        .arg(
            Arg::new("input")
                .help("Input GDScript file to format")
                .value_name("FILE")
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file (default: stdout)")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .help("Check if file is formatted (exit code 1 if not)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_content = match matches.get_one::<String>("input") {
        Some(file_path) => {
            let path = PathBuf::from(file_path);
            fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?
        }
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .map_err(|e| format!("Failed to read from stdin: {}", e))?;
            buffer
        }
    };

    let formatted_content = format_gdscript(&input_content)?;

    if matches.get_flag("check") {
        if input_content != formatted_content {
            eprintln!("File is not formatted");
            std::process::exit(1);
        }
        println!("File is formatted");
    } else {
        match matches.get_one::<String>("output") {
            Some(output_file) => {
                fs::write(output_file, formatted_content)
                    .map_err(|e| format!("Failed to write to file {}: {}", output_file, e))?;
            }
            None => {
                print!("{}", formatted_content);
            }
        }
    }

    Ok(())
}
