use std::{fs, path::Path, sync::LazyLock};

use serde::{Deserialize, Serialize};

use crate::{location::Location, scene::Scene};

/// The container for persistent plugin settings.
#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    /// Determines if Dive to the Heart should appear with stage hazards disabled.
    pub override_gimmick_off: ConfigParameter<bool>,

    /// Determines if Dive to the Heart should not appear with stage hazards enabled.
    pub override_gimmick_on: ConfigParameter<bool>,

    /// The starting location of the stage.
    pub default_location: ConfigParameter<Location>,

    /// The fallback stained-glass design.
    pub default_scene: ConfigParameter<Scene>,
}

impl Config {
    /// The path to the configuration file.
    const PATH: &str = "sd:/ultimate/stage_divetoheart/config.toml";

    /// Creates a new `Config`.
    fn new() -> Self {
        let path = Path::new(Self::PATH);

        if path.exists() {
            return Self::read(path);
        }

        let parent = path.parent().unwrap();

        if !parent.exists() {
            fs::create_dir_all(parent).unwrap();
        }

        let config = Self::default();

        config.write(path);

        config
    }

    /// Returns a reference to the lazily initialized `Config`.
    pub const fn get() -> &'static LazyLock<Self> {
        static INSTANCE: LazyLock<Config> = LazyLock::new(Config::new);

        &INSTANCE
    }

    /// Reads and deserializes the configuration file from disk.
    fn read<P: AsRef<Path>>(path: P) -> Self {
        match fs::read_to_string(&path) {
            Ok(string) => match toml::from_str(&string) {
                Ok(config) => return config,
                Err(error) => {
                    eprintln!(
                        "[{}] Failed to deserialize configuration file: {error}",
                        module_path!(),
                    );
                }
            },
            Err(error) => {
                eprintln!(
                    "[{}] Failed to read configuration file: {error}",
                    module_path!(),
                );
            }
        }

        Self::default()
    }

    /// Serializes and writes the current configuration to disk.
    fn write<P: AsRef<Path>>(&self, path: P) {
        match toml::to_string(self) {
            Ok(toml) => match fs::write(&path, toml) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!(
                        "[{}] Failed to write configuration file: {error}",
                        module_path!(),
                    );
                }
            },
            Err(error) => {
                eprintln!(
                    "[{}] Failed to serialize configuration: {error}",
                    module_path!(),
                );
            }
        }
    }
}

/// A table for a [`Config`] containing fields for each form of the stage.
#[derive(Default, Serialize, Deserialize)]
pub struct ConfigParameter<T> {
    /// The field for the stage's normal form.
    pub normal: T,

    /// The field for the stage's Battlefield form.
    pub battle: T,

    /// The field for the stage's Ω form.
    pub end: T,
}
