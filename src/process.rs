use std::process::Command;

/**
 * Execute an command and return result of stderr (Err) or stdout (Ok)
 */
pub fn exec(command: &mut Command) -> Result<String, String> {
    let output = command.output().map_err(|e| e.to_string())?;

    return Ok(String::from_utf8(output.stdout).unwrap());
}
