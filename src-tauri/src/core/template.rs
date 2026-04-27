use anyhow::{Context, Result};
use log::{error, info};
use regex::Regex;
use semver::Version;
use serde_json::json;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

pub static SCRIPT_ASK: &[u8] = include_bytes!("../../scripts/ask.js");


/// Struct representing the template with the script data.
#[derive(Debug)]
pub struct Template {
    pub ask: Vec<u8>,
}

impl Template {
    /// Creates a new Template instance, initializing it with the script data.
    pub fn new<P: AsRef<Path>>(template_dir: P) -> Self {
        let template_dir = template_dir.as_ref();
        let mut template = Template::default();

        let files = vec![(template_dir.join("ask.js"), &mut template.ask)];

        for (filename, _) in files {
            match update_or_create_file(&filename, SCRIPT_ASK) {
                Ok(updated) => {
                    if updated {
                        info!("Script updated or created: {}", filename.display());
                    } else {
                        info!("Script is up-to-date: {}", filename.display());
                    }
                }
                Err(e) => {
                    error!("Failed to process script, {}: {}", filename.display(), e);
                }
            }
        }

        template
    }
}

impl Default for Template {
    fn default() -> Template {
        Template {
            ask: Vec::from(SCRIPT_ASK),
        }
    }
}

/// Reads the version information from the given data.
fn read_version_info(data: &[u8]) -> Result<serde_json::Value> {
    let content = String::from_utf8_lossy(data);
    let re_name = Regex::new(r"@name\s+(.*?)\n").context("Failed to compile name regex")?;
    let re_version =
        Regex::new(r"@version\s+(.*?)\n").context("Failed to compile version regex")?;
    let re_url = Regex::new(r"@url\s+(.*?)\n").context("Failed to compile url regex")?;

    let name = re_name
        .captures(&content)
        .and_then(|cap| cap.get(1))
        .map_or(String::new(), |m| m.as_str().trim().to_string());

    let version = re_version
        .captures(&content)
        .and_then(|cap| cap.get(1))
        .map_or(String::new(), |m| m.as_str().trim().to_string());

    let url = re_url
        .captures(&content)
        .and_then(|cap| cap.get(1))
        .map_or(String::new(), |m| m.as_str().trim().to_string());

    let json_data = json!({
        "name": name,
        "version": version,
        "url": url,
    });

    Ok(json_data)
}

/// Reads the contents of the given file.
fn read_file_contents<P: AsRef<Path>>(filename: P) -> Result<Vec<u8>> {
    let filename = filename.as_ref();
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

/// Writes the given data to the specified file.
fn write_file_contents<P: AsRef<Path>>(filename: P, data: &[u8]) -> Result<()> {
    let filename = filename.as_ref();
    let mut file = File::create(filename)?;
    file.write_all(data)?;
    Ok(())
}

/// Creates the necessary directories for the specified file path.
fn create_dir<P: AsRef<Path>>(filename: P) -> Result<()> {
    let filename = filename.as_ref();
    if let Some(parent) = filename.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

/// Updates the file if the new data has a newer version or if version info is missing,
/// or creates the file if it doesn't exist.
fn update_or_create_file<P: AsRef<Path>>(filename: P, new_data: &[u8]) -> Result<bool> {
    let filename = filename.as_ref();

    // Ensure directory exists
    create_dir(filename)?;

    let current_data = read_file_contents(filename);

    match current_data {
        Ok(current_data) => {
            let new_info = read_version_info(new_data)?;
            let current_info = read_version_info(&current_data);

            match (
                new_info.get("version").and_then(|v| v.as_str()),
                current_info,
            ) {
                (Some(new_version), Ok(current_info)) => {
                    let current_version = current_info
                        .get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    if current_version.is_empty()
                        || Version::parse(new_version)? > Version::parse(current_version)?
                    {
                        write_file_contents(filename, new_data)?;
                        info!("{} → {}", current_version, new_version);
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
                // If there is an error reading current version info, update the file
                (Some(_), Err(_)) => {
                    write_file_contents(filename, new_data)?;
                    Ok(true)
                }
                (None, _) => {
                    // If there is an error reading new version info, don't update the file
                    Ok(false)
                }
            }
        }
        Err(_) => {
            // If there is an error reading the current file, create a new file
            write_file_contents(filename, new_data)?;
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper: add tempfile as dev-dependency by using std::env::temp_dir instead
    fn temp_dir_path() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "chatgpt_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn script_with_version(name: &str, version: &str, url: &str) -> Vec<u8> {
        format!(
            "/**\n * @name {}\n * @version {}\n * @url {}\n */\nwindow.X = 1;\n",
            name, version, url
        )
        .into_bytes()
    }

    // ── read_version_info ──────────────────────────────────────────────────────

    #[test]
    fn test_read_version_info_all_fields() {
        let data = script_with_version("ask.js", "1.2.3", "https://example.com");
        let info = read_version_info(&data).expect("should parse version info");
        assert_eq!(info["name"], "ask.js");
        assert_eq!(info["version"], "1.2.3");
        assert_eq!(info["url"], "https://example.com");
    }

    #[test]
    fn test_read_version_info_missing_fields_returns_empty_strings() {
        let data = b"window.X = 1;";
        let info = read_version_info(data).expect("should return empty strings for missing fields");
        assert_eq!(info["name"], "");
        assert_eq!(info["version"], "");
        assert_eq!(info["url"], "");
    }

    #[test]
    fn test_read_version_info_partial_fields() {
        let data = b"/**\n * @name myScript\n */\nwindow.X = 1;\n";
        let info = read_version_info(data).expect("should parse partial info");
        assert_eq!(info["name"], "myScript");
        assert_eq!(info["version"], "");
    }

    #[test]
    fn test_read_version_info_whitespace_trimmed() {
        let data = b"/**\n * @name   spaced  \n * @version  2.0.0  \n */\n";
        let info = read_version_info(data).expect("should trim whitespace");
        assert_eq!(info["name"], "spaced");
        assert_eq!(info["version"], "2.0.0");
    }

    // ── read_file_contents / write_file_contents ───────────────────────────────

    #[test]
    fn test_write_and_read_file_roundtrip() {
        let dir = temp_dir_path();
        let path = dir.join("test.txt");
        let data = b"hello world";

        write_file_contents(&path, data).expect("write should succeed");
        let read = read_file_contents(&path).expect("read should succeed");
        assert_eq!(read, data);

        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn test_read_file_contents_missing_file_returns_error() {
        let path = std::path::Path::new("/nonexistent/path/file.txt");
        assert!(read_file_contents(path).is_err());
    }

    // ── create_dir ────────────────────────────────────────────────────────────

    #[test]
    fn test_create_dir_creates_nested_directories() {
        let base = temp_dir_path();
        let nested = base.join("a").join("b").join("c").join("file.txt");
        create_dir(&nested).expect("create_dir should succeed for nested path");
        assert!(nested.parent().unwrap().exists());
        fs::remove_dir_all(base).ok();
    }

    #[test]
    fn test_create_dir_already_exists_is_ok() {
        let dir = temp_dir_path();
        let file = dir.join("existing.txt");
        // Parent already exists; should be fine
        create_dir(&file).expect("create_dir should succeed when parent already exists");
        fs::remove_dir_all(dir).ok();
    }

    // ── update_or_create_file ─────────────────────────────────────────────────

    #[test]
    fn test_update_or_create_file_creates_when_missing() {
        let dir = temp_dir_path();
        let path = dir.join("new.js");
        let data = script_with_version("new.js", "1.0.0", "https://example.com");

        let updated = update_or_create_file(&path, &data).expect("should create file");
        assert!(updated, "should report file as created");
        assert!(path.exists());
        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn test_update_or_create_file_upgrades_older_version() {
        let dir = temp_dir_path();
        let path = dir.join("script.js");

        let old = script_with_version("script.js", "0.9.0", "https://example.com");
        let new = script_with_version("script.js", "1.0.0", "https://example.com");

        write_file_contents(&path, &old).unwrap();
        let updated = update_or_create_file(&path, &new).expect("should update");
        assert!(updated, "should report file as updated to newer version");

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("1.0.0"));
        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn test_update_or_create_file_skips_same_version() {
        let dir = temp_dir_path();
        let path = dir.join("script.js");
        let data = script_with_version("script.js", "1.0.0", "https://example.com");

        write_file_contents(&path, &data).unwrap();
        let updated = update_or_create_file(&path, &data).expect("should not update");
        assert!(!updated, "same version should not trigger update");
        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn test_update_or_create_file_skips_older_incoming_version() {
        let dir = temp_dir_path();
        let path = dir.join("script.js");

        let current = script_with_version("script.js", "2.0.0", "https://example.com");
        let older = script_with_version("script.js", "1.0.0", "https://example.com");

        write_file_contents(&path, &current).unwrap();
        let updated = update_or_create_file(&path, &older).expect("should not downgrade");
        assert!(!updated, "older version should not replace newer file");
        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn test_update_or_create_file_overwrites_when_no_version_in_existing() {
        let dir = temp_dir_path();
        let path = dir.join("script.js");

        // existing file has no version tag
        write_file_contents(&path, b"window.X = 1;").unwrap();
        let new = script_with_version("script.js", "1.0.0", "https://example.com");

        let updated = update_or_create_file(&path, &new).expect("should overwrite");
        assert!(updated, "missing version in existing file should trigger overwrite");
        fs::remove_dir_all(dir).ok();
    }

    // ── Template::default ─────────────────────────────────────────────────────

    #[test]
    fn test_template_default_contains_ask_script() {
        let t = Template::default();
        assert!(
            !t.ask.is_empty(),
            "default template should embed the ask.js bytes"
        );
        // Verify it starts with the JS comment header
        let content = String::from_utf8_lossy(&t.ask);
        assert!(content.contains("ChatAsk"), "should contain ChatAsk class");
    }

    // ── Template::new ─────────────────────────────────────────────────────────

    #[test]
    fn test_template_new_creates_ask_js() {
        let dir = temp_dir_path();
        Template::new(&dir);
        assert!(dir.join("ask.js").exists(), "Template::new should create ask.js");
        fs::remove_dir_all(dir).ok();
    }
}
