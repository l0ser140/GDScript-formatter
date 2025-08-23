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
    version = "0.1.0",
    long_about = "Format GDScript files with consistent style and indentation. \
    By default, the formatter overwrites input files with the formatted code. \
    Use --stdout to output to standard output instead."
)]
struct Args {
    #[arg(
        help = "Input GDScript file to format. If not provided, reads from stdin and outputs to stdout",
        value_name = "FILE"
    )]
    input: Option<PathBuf>,
    #[arg(
        long,
        help = "Output formatted code to stdout instead of overwriting the input file. \
        This flag is ignored when reading from stdin (stdout is always used)"
    )]
    stdout: bool,
    #[arg(
        short,
        long,
        help = "Check if the file is already formatted without making changes. \
        Exits with code 0 if the file is already formatted and 1 if it's not formatted"
    )]
    check: bool,
    #[arg(
        long,
        help = "Use spaces for indentation instead of tabs. \
        The number of spaces is controlled by --indent-size"
    )]
    use_spaces: bool,
    #[arg(
        long,
        help = "Number of spaces to use for each indentation level when --use-spaces is enabled. \
        Has no effect without the --use-spaces flag.",
        default_value = "4",
        value_name = "NUM"
    )]
    indent_size: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input_content = match &args.input {
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
            eprintln!("The file is not formatted");
            std::process::exit(1);
        }
        println!("File is formatted");
    } else {
        match (args.input.as_ref(), args.stdout) {
            // If there's no input file, always output to stdout
            (None, _) => {
                print!("{}", formatted_content);
            }
            // We're reading from a file and the --stdout flag is on: we output to stdout
            (Some(_), true) => {
                print!("{}", formatted_content);
            }
            // We're reading from a file without the --stdout flag: we overwrite the input file
            (Some(input_file), false) => {
                fs::write(input_file, formatted_content).map_err(|e| {
                    format!("Failed to write to file {}: {}", input_file.display(), e)
                })?;
            }
        }
    }

    Ok(())
}
