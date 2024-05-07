use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repos {
    enabled: bool,
    pub remote: String,
    branch: String,
    group: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitpSetting {
    user: User,
    comments: HashMap<String, String>,
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
            repos: Vec::new(),
        }
    }
}

// pub fn load(mut gitp_setting: GitpSetting) -> Result<GitpSetting, Box<dyn Error>> {
pub fn load() -> Result<GitpSetting, Box<dyn Error>> {
    let mut file = File::open("gitp_setting.yml")?;
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
