use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use clap::Parser;

use gdscript_formatter::{formatter::format_gdscript_with_config, FormatterConfig};

#[derive(Parser)]
#[clap(
    about = "A GDScript code formatter using Topiary and Tree-sitter",
    version = "0.1.0"
)]
struct Args {
    #[arg(help = "Input GDScript file to format", value_name = "FILE")]
    input: Option<PathBuf>,
    #[arg(
        short,
        long,
        help = "Output file (default: stdout)",
        value_name = "FILE"
    )]
    output: Option<PathBuf>,
    #[arg(short, long, help = "Check if file is formatted (exit code 1 if not)")]
    check: bool,
    #[arg(long, help = "Use spaces for indentation")]
    use_spaces: bool,
    #[arg(
        long,
        help = "Number of space to use as indentation when --use-space is present",
        default_value = "4"
    )]
    indent_size: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input_content = match args.input {
        Some(file_path) => fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?,
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .map_err(|e| format!("Failed to read from stdin: {}", e))?;
            buffer
        }
    };

    let config = FormatterConfig {
        indent_size: args.indent_size,
        use_spaces: args.use_spaces,
    };

    let formatted_content = format_gdscript_with_config(&input_content, &config)?;

    if args.check {
        if input_content != formatted_content {
            eprintln!("File is not formatted");
            std::process::exit(1);
        }
        println!("File is formatted");
    } else {
        match args.output {
            Some(output_file) => {
                fs::write(&output_file, formatted_content).map_err(|e| {
                    format!("Failed to write to file {}: {}", output_file.display(), e)
                })?;
            }
            None => {
                print!("{}", formatted_content);
            }
        }
    }

    Ok(())
}
