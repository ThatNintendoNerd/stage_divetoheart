use std::{fs, path::Path, sync::LazyLock};

use serde::{Deserialize, Serialize};

use crate::{location::Location, scene::Scene};

/// The container for deserializable plugin settings.
#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    /// Determines if Dive to the Heart can appear with stage hazards disabled.
    pub override_gimmick_off: ConfigParameter<bool>,

    /// Determines if Dive to the Heart cannot appear with stage hazards enabled.
    pub override_gimmick_on: ConfigParameter<bool>,

    /// Determines the starting location of the stage.
    pub default_location: ConfigParameter<Location>,

    /// Determines the fallback stained-glass design.
    pub default_scene: ConfigParameter<Scene>,
}

impl Config {
    /// Returns a reference to a `LazyLock` containing the current instance of `Config`.
    pub const fn get() -> &'static LazyLock<Self> {
        static INSTANCE: LazyLock<Config> = LazyLock::new(|| {
            let path = Path::new(Config::path());

            if path.exists() {
                return Config::read(path);
            }

            let parent = path.parent().unwrap();

            if !parent.exists() {
                fs::create_dir_all(parent).unwrap();
            }

            let config = Config::default();

            config.write(path);

            config
        });

        &INSTANCE
    }

    /// Reads and deserializes the configuration file from disk.
    fn read<P: AsRef<Path>>(path: P) -> Self {
        match fs::read_to_string(&path) {
            Ok(string) => match toml::from_str(&string) {
                Ok(config) => return config,
                Err(error) => {
                    eprintln!(
                        "[{}] Failed to parse TOML file data from '{}': {}",
                        module_path!(),
                        path.as_ref().display(),
                        error,
                    );
                }
            },
            Err(error) => {
                eprintln!(
                    "[{}] Failed to read TOML file data from '{}': {}",
                    module_path!(),
                    path.as_ref().display(),
                    error,
                );
            }
        }

        Self::default()
    }

    /// Serializes and writes the current configuration to disk.
    fn write<P: AsRef<Path>>(&self, path: P) {
        match toml::to_string(self) {
            Ok(toml) => {
                if let Err(error) = fs::write(&path, toml) {
                    eprintln!(
                        "[{}] Failed to write TOML file data to '{}': {}",
                        module_path!(),
                        path.as_ref().display(),
                        error,
                    );
                }
            }
            Err(error) => {
                eprintln!(
                    "[{}] Failed to serialize configuration: {}",
                    module_path!(),
                    error,
                );
            }
        }
    }

    /// Returns the path to the configuration file.
    fn path() -> &'static str {
        "sd:/ultimate/stage_divetoheart/config.toml"
    }
}

/// The field type for a `Config` containing settings for each form of the stage.
#[derive(Default, Serialize, Deserialize)]
pub struct ConfigParameter<T> {
    /// The setting for the stage's normal form.
    pub normal: T,

    /// The setting for the stage's Battlefield form.
    pub battle: T,

    /// The setting for the stage's Omega form.
    pub end: T,
}
