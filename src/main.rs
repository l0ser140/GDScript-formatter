use std::{
    env, fs,
    io::{self, IsTerminal, Read, Write},
    path::PathBuf,
};

use clap::{CommandFactory, Parser};

use gdscript_formatter::{FormatterConfig, formatter::format_gdscript_with_config};

#[derive(Parser)]
#[clap(
    about = "A GDScript code formatter using Topiary and Tree-sitter",
    // Use the version number directly from Cargo.toml at compile time
    version = env!("CARGO_PKG_VERSION"),
    long_about = "Format GDScript files with consistent style and indentation. \
    By default, the formatter overwrites input files with the formatted code. \
    Use --stdout to output to standard output instead."
)]
struct Args {
    #[arg(
        help = "Input GDScript file(s) to format. If no file path is provided, the program reads from standard input and outputs to standard output.",
        value_name = "FILES"
    )]
    input: Vec<PathBuf>,
    #[arg(
        long,
        help = "Output formatted code to stdout instead of overwriting the input file. \
        If multiple input files are provided, each file's content is preceded by a comment indicating the file name, with the form \
        #--file:<file_path> \
        This flag is ignored when reading from stdin (stdout is always used)."
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
    #[arg(
        long,
        help = "Reorder source-level declarations (signals, properties, methods, etc.) according to the official GDScript style guide. \
        This is optional and applies after the main formatting pass."
    )]
    reorder_code: bool,
    #[arg(
        short,
        long,
        help = "Enable safe mode. This mode ensures that after formatting, the code still has the same syntax and structure \
        as before. If not, formatting is canceled. This offers some protection against the formatter failing on new syntax \
        at the cost of a small little extra running time. Currently incompatible with --reorder-code.",
        conflicts_with = "reorder_code"
    )]
    safe: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // If there are no arguments and nothing piped from stdin, print the help message
    if env::args().len() == 1 && io::stdin().is_terminal() {
        Args::command().print_help()?;
        println!();
        return Ok(());
    }

    let args = Args::parse();

    let config = FormatterConfig {
        indent_size: args.indent_size,
        use_spaces: args.use_spaces,
        reorder_code: args.reorder_code,
        safe: args.safe,
    };

    if args.input.is_empty() {
        let mut input_content = String::new();
        io::stdin()
            .read_to_string(&mut input_content)
            .map_err(|error| format!("Failed to read from stdin: {}", error))?;

        let formatted_content = format_gdscript_with_config(&input_content, &config)?;

        if args.check {
            if input_content != formatted_content {
                eprintln!("The input passed via stdin is not formatted");
                std::process::exit(1);
            } else {
                eprintln!("The input passed via stdin is already formatted");
            }
        } else {
            print!("{}", formatted_content);
        }

        return Ok(());
    }

    let input_gdscript_files: Vec<&PathBuf> = args
        .input
        .iter()
        .filter(|path| path.extension().map_or(false, |ext| ext == "gd"))
        .collect();

    if input_gdscript_files.is_empty() {
        eprintln!(
            "Error: No GDScript files found in the arguments provided. Please provide at least one .gd file."
        );
        std::process::exit(1);
    }

    let total_files = input_gdscript_files.len();
    let mut all_formatted = true;

    for (index, file_path) in input_gdscript_files.iter().enumerate() {
        let file_number = index + 1;
        terminal_clear_line();
        eprint!("\rFormatting file {}/{}", file_number, total_files);
        io::stdout().flush().unwrap();

        let input_content = fs::read_to_string(file_path)
            .map_err(|error| format!("Failed to read file {}: {}", file_path.display(), error))?;

        let formatted_content = format_gdscript_with_config(&input_content, &config)?;

        if args.check {
            if input_content != formatted_content {
                all_formatted = false;
            }
        } else if args.stdout {
            // Clear the current line before printing formatted files to stdout, to erase the "Formatting file ..." message
            terminal_clear_line();
            // A little bit hacky, but because terminals by default output both stdout and stderr
            // we need to return carriage to the start to print formatted output from the start of the line
            eprint!("\r");
            // If there are multiple input files we still allow stdout but we print a separator
            if total_files > 1 {
                println!("#--file:{}", file_path.display());
            }
            print!("{}", formatted_content);
        } else {
            fs::write(file_path, formatted_content)
                .map_err(|e| format!("Failed to write to file {}: {}", file_path.display(), e))?;
        }
    }

    if args.check {
        if all_formatted {
            terminal_clear_line();
            eprintln!("\rAll {} file(s) are formatted", total_files);
        } else {
            terminal_clear_line();
            eprintln!("\rSome files are not formatted");
            std::process::exit(1);
        }
    } else if !args.stdout {
        terminal_clear_line();
        eprintln!(
            "\rFormatted {} file{}",
            total_files,
            if total_files == 1 { "" } else { "s" }
        );
    }

    Ok(())
}

fn terminal_clear_line() {
    eprint!("\r{}", " ".repeat(80));
}
