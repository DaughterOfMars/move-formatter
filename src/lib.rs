use std::path::PathBuf;

use movefmt::GetOptsOptions;
use napi::Status;
use napi_derive::napi;

const CONFIG_FILENAME: &str = "movefmt.toml";

#[napi]
/// Format a move file as a string.
pub fn format_string(s: String, config_path: Option<String>) -> napi::Result<String> {
    format(s, config_path.map(Into::into)).map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
}

#[napi]
// Search up the hierarchy for a movefmt.toml config file.
pub fn get_config_path(path: String) -> napi::Result<Option<String>> {
    find_config_file(path.into())
        .map(|p| p.map(|p| p.to_string_lossy().into_owned()))
        .map_err(|e| napi::Error::new(Status::GenericFailure, e.to_string()))
}

fn format(s: String, config_path: Option<PathBuf>) -> Result<String, Box<dyn std::error::Error>> {
    Ok(movefmt::format_text(
        &s,
        &GetOptsOptions {
            quiet: true,
            config_path,
            ..Default::default()
        },
    )?)
}

fn find_config_file(mut path: PathBuf) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    loop {
        match path.file_name() {
            Some(_) if path.is_dir() && path.join(CONFIG_FILENAME).exists() => {
                return Ok(Some(path.join(CONFIG_FILENAME)));
            }
            _ => {
                if !path.pop() {
                    return Ok(None);
                }
            }
        }
    }
}
