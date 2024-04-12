use serde::{Deserialize, Serialize};

/// The starting locations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum Location {
    /// Hollow Bastion.
    #[default]
    #[serde(rename = "Hollow Bastion")]
    HollowBastion,

    /// Dive to the Heart.
    #[serde(rename = "Dive to the Heart")]
    DiveToHeart,

    /// A random starting location between Hollow Bastion and Dive to the Heart.
    Random,
}
