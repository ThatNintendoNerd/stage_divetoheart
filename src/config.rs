use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::scene::{Location, Scene};

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let path = Path::new("sd:/ultimate/config_dth.toml");

    if !path.is_file() {
        let config = Config::new();

        if config.save(path).is_err() {
            println!("[stage_divetoheart::config] Failed to create configuration file '{}'. Using default settings...", path.display());

            return config;
        }
    }

    let config = fs::read_to_string(&path).ok().and_then(|string| toml::from_str(&string).ok());

    if let Some(config) = config {
        config
    } else {
        println!("[stage_divetoheart::config] Failed to parse TOML data from file '{}'. Using default settings...", path.display());

        Config::new()
    }
});

#[derive(Deserialize, Serialize)]
pub struct HazardsOffOverride {
    #[serde(default)]
    pub normal: bool,

    #[serde(default)]
    pub battle: bool,

    #[serde(default)]
    pub end: bool,
}

#[derive(Deserialize, Serialize)]
pub struct DefaultLocation {
    #[serde(default)]
    pub normal: Location,

    #[serde(default)]
    pub battle: Location,

    #[serde(default)]
    pub end: Location,
}

#[derive(Deserialize, Serialize)]
pub struct DefaultStation {
    #[serde(default)]
    pub normal: Scene,

    #[serde(default)]
    pub battle: Scene,

    #[serde(default)]
    pub end: Scene,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub hazards_off_override: HazardsOffOverride,
    pub default_location: DefaultLocation,
    pub default_station: DefaultStation,
}

impl Config {
    pub fn new() -> Self {
        Self {
            hazards_off_override: HazardsOffOverride {
                normal: false,
                battle: false,
                end: false,
            },
            default_location: DefaultLocation {
                normal: Location("Hollow Bastion".to_string()),
                battle: Location("Hollow Bastion".to_string()),
                end: Location("Hollow Bastion".to_string()),
            },
            default_station: DefaultStation {
                normal: Scene::Random,
                battle: Scene::Random,
                end: Scene::Random,
            },
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), io::Error> {
        let config = toml::to_vec(&self).unwrap_or_default();

        File::create(&path)?.write_all(&config)?;

        Ok(())
    }
}
