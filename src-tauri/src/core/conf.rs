use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};
use tauri::{AppHandle, Manager, Theme};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConf {
    pub theme: String,
    pub stay_on_top: bool,
    pub ask_mode: bool,
    pub mac_titlebar_hidden: bool,
}

impl AppConf {
    pub fn new() -> Self {
        Self {
            theme: "system".to_string(),
            stay_on_top: false,
            ask_mode: false,
            #[cfg(target_os = "macos")]
            mac_titlebar_hidden: true,
            #[cfg(not(target_os = "macos"))]
            mac_titlebar_hidden: false,
        }
    }

    pub fn get_conf_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = app
            .path()
            .config_dir()?
            .join("com.nofwl.chatgpt")
            .join("config.json");
        Ok(config_dir)
    }

    pub fn get_scripts_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let scripts_dir = app
            .path()
            .config_dir()?
            .join("com.nofwl.chatgpt")
            .join("scripts");
        Ok(scripts_dir)
    }

    pub fn load_script(app: &AppHandle, filename: &str) -> String {
        let script_file = Self::get_scripts_path(app).unwrap().join(filename);
        fs::read_to_string(script_file).unwrap_or_else(|_| "".to_string())
    }

    pub fn load(app: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::get_conf_path(app)?;

        if !path.exists() {
            let config = Self::new();
            config.save(app)?;
            return Ok(config);
        }

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Result<AppConf, _> = serde_json::from_str(&contents);

        // Handle conditional fields and fallback to defaults if necessary
        if let Err(e) = &config {
            error!("[conf::load] {}", e);
            let mut default_config = Self::new();
            default_config = default_config.amend(serde_json::from_str(&contents)?)?;
            default_config.save(app)?;
            return Ok(default_config);
        }

        Ok(config?)
    }

    pub fn save(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {

        let path = Self::get_conf_path(app)?;

        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }

        let mut file = File::create(path)?;
        let contents = serde_json::to_string_pretty(self)?;
        // dbg!(&contents);
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn amend(self, json: Value) -> Result<Self, serde_json::Error> {
        let val = serde_json::to_value(self)?;
        let mut config: BTreeMap<String, Value> = serde_json::from_value(val)?;
        let new_json: BTreeMap<String, Value> = serde_json::from_value(json)?;

        for (k, v) in new_json {
            config.insert(k, v);
        }

        let config_str = serde_json::to_string_pretty(&config)?;
        serde_json::from_str::<AppConf>(&config_str).map_err(|err| {
            error!("[conf::amend] {}", err);
            err
        })
    }

    pub fn get_theme(app: &AppHandle) -> Theme {
        let theme = Self::load(app).unwrap().theme;
        match theme.as_str() {
            "system" => match dark_light::detect() {
                dark_light::Mode::Dark => Theme::Dark,
                dark_light::Mode::Light => Theme::Light,
                dark_light::Mode::Default => Theme::Light,
            },
            "dark" => Theme::Dark,
            _ => Theme::Light,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_conf_json() -> Value {
        serde_json::json!({
            "theme": "system",
            "stay_on_top": false,
            "ask_mode": false,
            "mac_titlebar_hidden": false,
        })
    }

    #[test]
    fn test_app_conf_new_defaults() {
        let conf = AppConf::new();
        assert_eq!(conf.theme, "system");
        assert!(!conf.stay_on_top);
        assert!(!conf.ask_mode);
    }

    #[test]
    fn test_app_conf_amend_theme() {
        let conf = AppConf {
            theme: "system".to_string(),
            stay_on_top: false,
            ask_mode: false,
            mac_titlebar_hidden: false,
        };
        let updated = conf
            .amend(serde_json::json!({"theme": "dark"}))
            .expect("amend should succeed");
        assert_eq!(updated.theme, "dark");
        assert!(!updated.stay_on_top);
        assert!(!updated.ask_mode);
    }

    #[test]
    fn test_app_conf_amend_stay_on_top() {
        let conf = AppConf {
            theme: "light".to_string(),
            stay_on_top: false,
            ask_mode: false,
            mac_titlebar_hidden: false,
        };
        let updated = conf
            .amend(serde_json::json!({"stay_on_top": true}))
            .expect("amend should succeed");
        assert!(updated.stay_on_top);
        assert_eq!(updated.theme, "light");
    }

    #[test]
    fn test_app_conf_amend_ask_mode() {
        let conf = AppConf {
            theme: "dark".to_string(),
            stay_on_top: false,
            ask_mode: false,
            mac_titlebar_hidden: false,
        };
        let updated = conf
            .amend(serde_json::json!({"ask_mode": true}))
            .expect("amend should succeed");
        assert!(updated.ask_mode);
        assert_eq!(updated.theme, "dark");
    }

    #[test]
    fn test_app_conf_amend_multiple_fields() {
        let conf = AppConf {
            theme: "system".to_string(),
            stay_on_top: false,
            ask_mode: false,
            mac_titlebar_hidden: false,
        };
        let updated = conf
            .amend(serde_json::json!({"theme": "light", "stay_on_top": true, "ask_mode": true}))
            .expect("amend should succeed");
        assert_eq!(updated.theme, "light");
        assert!(updated.stay_on_top);
        assert!(updated.ask_mode);
    }

    #[test]
    fn test_app_conf_amend_empty_patch() {
        let conf = AppConf {
            theme: "dark".to_string(),
            stay_on_top: true,
            ask_mode: true,
            mac_titlebar_hidden: false,
        };
        let updated = conf
            .amend(serde_json::json!({}))
            .expect("amend with empty patch should succeed");
        assert_eq!(updated.theme, "dark");
        assert!(updated.stay_on_top);
        assert!(updated.ask_mode);
    }

    #[test]
    fn test_app_conf_serialization_roundtrip() {
        let conf = AppConf {
            theme: "light".to_string(),
            stay_on_top: true,
            ask_mode: false,
            mac_titlebar_hidden: true,
        };
        let json = serde_json::to_string(&conf).expect("serialize should succeed");
        let restored: AppConf = serde_json::from_str(&json).expect("deserialize should succeed");
        assert_eq!(restored.theme, conf.theme);
        assert_eq!(restored.stay_on_top, conf.stay_on_top);
        assert_eq!(restored.ask_mode, conf.ask_mode);
        assert_eq!(restored.mac_titlebar_hidden, conf.mac_titlebar_hidden);
    }

    #[test]
    fn test_app_conf_all_theme_values() {
        for theme in &["system", "light", "dark"] {
            let conf = AppConf {
                theme: "system".to_string(),
                stay_on_top: false,
                ask_mode: false,
                mac_titlebar_hidden: false,
            };
            let updated = conf
                .amend(serde_json::json!({"theme": theme}))
                .expect("amend theme should succeed");
            assert_eq!(&updated.theme, theme);
        }
    }

    #[test]
    fn test_app_conf_amend_does_not_mutate_original() {
        let original_theme = "system".to_string();
        let conf = AppConf {
            theme: original_theme.clone(),
            stay_on_top: false,
            ask_mode: false,
            mac_titlebar_hidden: false,
        };
        // amend consumes self, so we verify the updated value is independent
        let updated = conf
            .amend(serde_json::json!({"theme": "dark"}))
            .expect("amend should succeed");
        assert_eq!(updated.theme, "dark");
        assert_ne!(updated.theme, original_theme);
    }

    #[test]
    fn test_app_conf_default_json_roundtrip() {
        let json = default_conf_json();
        let conf: AppConf = serde_json::from_value(json).expect("should parse default conf");
        assert_eq!(conf.theme, "system");
        assert!(!conf.stay_on_top);
        assert!(!conf.ask_mode);
    }
}
