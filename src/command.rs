use ninput::{Buttons, Controller, ControllerStyle};

use crate::scene::Scene;

/// The evaluated controller buttons when selecting Hollow Bastion on the Stage Select screen.
pub struct StageSelectSecretCommand {
    /// The A button.
    a: Buttons,

    /// The L button.
    l: Buttons,

    /// The R button.
    r: Buttons,

    /// The left button.
    left: Buttons,

    /// The up button.
    up: Buttons,

    /// The right button.
    right: Buttons,

    /// The down button.
    down: Buttons,
}

impl StageSelectSecretCommand {
    /// Constructs a new instance of `StageSelectSecretCommand` from a set of buttons.
    fn from_full_key(buttons: Buttons) -> Self {
        Self {
            a: Buttons::A,
            l: Buttons::L,
            r: Buttons::R,
            left: if buttons.contains(Buttons::STICK_L_LEFT) {
                Buttons::STICK_L_LEFT
            } else {
                Buttons::LEFT
            },
            up: if buttons.contains(Buttons::STICK_L_UP) {
                Buttons::STICK_L_UP
            } else {
                Buttons::UP
            },
            right: if buttons.contains(Buttons::STICK_L_RIGHT) {
                Buttons::STICK_L_RIGHT
            } else {
                Buttons::RIGHT
            },
            down: if buttons.contains(Buttons::STICK_L_DOWN) {
                Buttons::STICK_L_DOWN
            } else {
                Buttons::DOWN
            },
        }
    }

    /// Constructs a new instance of `StageSelectSecretCommand` for the left Joy-Con.
    fn from_joy_left() -> Self {
        Self {
            a: Buttons::DOWN,
            l: Buttons::LEFT_SL,
            r: Buttons::LEFT_SR,
            left: Buttons::STICK_L_UP,
            up: Buttons::STICK_L_RIGHT,
            right: Buttons::STICK_L_DOWN,
            down: Buttons::STICK_L_LEFT,
        }
    }

    /// Constructs a new instance of `StageSelectSecretCommand` for the right Joy-Con.
    fn from_joy_right() -> Self {
        Self {
            a: Buttons::X,
            l: Buttons::RIGHT_SL,
            r: Buttons::RIGHT_SR,
            left: Buttons::STICK_R_DOWN,
            up: Buttons::STICK_R_LEFT,
            right: Buttons::STICK_R_UP,
            down: Buttons::STICK_R_RIGHT,
        }
    }

    /// Returns a stained-glass design variant depending on the given button set.
    pub fn scene(&self, buttons: Buttons) -> Scene {
        match buttons {
            buttons if buttons == self.sora() => Scene::Sora,
            buttons if buttons == self.riku() => Scene::Riku,
            buttons if buttons == self.roxas() => Scene::Roxas,
            buttons if buttons == self.xion() => Scene::Xion,
            buttons if buttons == self.terra() => Scene::Terra,
            buttons if buttons == self.ventus() => Scene::Ventus,
            buttons if buttons == self.aqua() => Scene::Aqua,
            _ => Scene::Random,
        }
    }

    /// Returns the button set for Sora's stained-glass design.
    fn sora(&self) -> Buttons {
        self.a | self.l
    }

    /// Returns the button set for Riku's stained-glass design.
    fn riku(&self) -> Buttons {
        self.a | self.l | self.up
    }

    /// Returns the button set for Roxas's stained-glass design.
    fn roxas(&self) -> Buttons {
        self.a | self.l | self.left
    }

    /// Returns the button set for Xion's stained-glass design.
    fn xion(&self) -> Buttons {
        self.a | self.l | self.down
    }

    /// Returns the button set for Terra's stained-glass design.
    fn terra(&self) -> Buttons {
        self.a | self.l | self.r | self.up
    }

    /// Returns the button set for Ventus's stained-glass design.
    fn ventus(&self) -> Buttons {
        self.a | self.l | self.r | self.right
    }

    /// Returns the button set for Aqua's stained-glass design.
    fn aqua(&self) -> Buttons {
        self.a | self.l | self.r | self.down
    }
}

impl From<&Controller> for StageSelectSecretCommand {
    fn from(controller: &Controller) -> Self {
        match controller.controller_style {
            ControllerStyle::LeftJoycon => Self::from_joy_left(),
            ControllerStyle::RightJoycon => Self::from_joy_right(),
            _ => Self::from_full_key(controller.buttons),
        }
    }
}
