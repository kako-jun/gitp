use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Repos {
    pub enabled: bool,
    pub remote: String,
    pub branch: String,
    pub group: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitpSetting {
    pub user: User,
    pub comments: HashMap<String, String>,
    #[serde(default)]
    pub config: HashMap<String, String>,
    pub repos: Vec<Repos>,
}

impl GitpSetting {
    pub fn default() -> GitpSetting {
        GitpSetting {
            user: User {
                name: String::from(""),
                email: String::from(""),
            },
            comments: HashMap::new(),
            config: HashMap::new(),
            repos: Vec::new(),
        }
    }
}

// pub fn load(mut gitp_setting: GitpSetting) -> Result<GitpSetting, Box<dyn Error>> {
pub fn load() -> Result<GitpSetting, Box<dyn Error>> {
    // Try .yaml first, then .yml (same logic as gitp.sh)
    let file_result = File::open("gitp_setting.yaml").or_else(|_| File::open("gitp_setting.yml"));

    let mut file = match file_result {
        Ok(f) => f,
        Err(_) => {
            return Err("Both gitp_setting.yaml and gitp_setting.yml do not exist.".into());
        }
    };

    let mut yaml_text = String::new();
    file.read_to_string(&mut yaml_text)?;

    // gitp_setting = serde_yaml::from_str(&yaml_text)?;
    let mut gitp_setting = GitpSetting::default();
    if !yaml_text.is_empty() {
        if let Ok(new_gitp_setting) = serde_yaml::from_str(&yaml_text) {
            gitp_setting = new_gitp_setting;
        }
    }

    Ok(gitp_setting)
}
