use crate::FormatterConfig;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn format_gdscript(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    format_gdscript_with_config(content, &FormatterConfig::default())
}

pub fn format_gdscript_with_config(
    content: &str,
    _config: &FormatterConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    // TEMP: Get the project root directory where our config files are located from cwd
    let project_root = std::env::current_dir()?;
    let config_file = project_root.join("config").join("languages.ncl");
    let queries_dir = project_root.join("queries");

    // Build and spawn command to run Topiary
    let mut topiary_command = Command::new("topiary");
    topiary_command
        .arg("format")
        .arg("--language")
        .arg("gdscript")
        .arg("--configuration")
        .arg(&config_file)
        .env("TOPIARY_LANGUAGE_DIR", &queries_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child_process = topiary_command.spawn().map_err(|e| {
        format!(
            "Failed to spawn topiary process: {}. Make sure 'topiary' is installed and in PATH.",
            e
        )
    })?;

    // Write input to stdin
    if let Some(stdin) = child_process.stdin.take() {
        let mut stdin = stdin;
        stdin.write_all(content.as_bytes())?;
    }
    let output = child_process.wait_with_output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Topiary formatting failed: {}", error_msg).into());
    }

    let formatted_content = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to parse topiary output as UTF-8: {}", e))?;

    Ok(formatted_content)
}
