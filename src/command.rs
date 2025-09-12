use ninput::{Buttons, Controller, ControllerStyle};

use crate::scene::Scene;

/// The evaluated controller buttons when selecting Hollow Bastion on the Stage Select screen.
pub struct StageSelectSecretCommand {
    /// The A Button.
    a: Buttons,

    /// The L Button.
    l: Buttons,

    /// The R Button.
    r: Buttons,

    /// The ← Button.
    left: Buttons,

    /// The ↑ Button.
    up: Buttons,

    /// The → Button.
    right: Buttons,

    /// The ↓ Button.
    down: Buttons,
}

impl StageSelectSecretCommand {
    /// The button map for the Joy-Con (L).
    const JOY_LEFT: Self = Self {
        a: Buttons::DOWN,
        l: Buttons::LEFT_SL,
        r: Buttons::LEFT_SR,
        left: Buttons::STICK_L_UP,
        up: Buttons::STICK_L_RIGHT,
        right: Buttons::STICK_L_DOWN,
        down: Buttons::STICK_L_LEFT,
    };

    /// The button map for the Joy-Con (R).
    const JOY_RIGHT: Self = Self {
        a: Buttons::X,
        l: Buttons::RIGHT_SL,
        r: Buttons::RIGHT_SR,
        left: Buttons::STICK_R_DOWN,
        up: Buttons::STICK_R_LEFT,
        right: Buttons::STICK_R_UP,
        down: Buttons::STICK_R_RIGHT,
    };

    /// Creates a new `StageSelectSecretCommand`.
    const fn new(buttons: Buttons) -> Self {
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

    /// Returns a stained-glass design based on the given button combination.
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

    /// Returns the button combination for Sora's stained-glass design.
    fn sora(&self) -> Buttons {
        self.a | self.l
    }

    /// Returns the button combination for Riku's stained-glass design.
    fn riku(&self) -> Buttons {
        self.a | self.l | self.up
    }

    /// Returns the button combination for Roxas's stained-glass design.
    fn roxas(&self) -> Buttons {
        self.a | self.l | self.left
    }

    /// Returns the button combination for Xion's stained-glass design.
    fn xion(&self) -> Buttons {
        self.a | self.l | self.down
    }

    /// Returns the button combination for Terra's stained-glass design.
    fn terra(&self) -> Buttons {
        self.a | self.l | self.r | self.up
    }

    /// Returns the button combination for Ventus's stained-glass design.
    fn ventus(&self) -> Buttons {
        self.a | self.l | self.r | self.right
    }

    /// Returns the button combination for Aqua's stained-glass design.
    fn aqua(&self) -> Buttons {
        self.a | self.l | self.r | self.down
    }
}

impl From<&Controller> for StageSelectSecretCommand {
    fn from(controller: &Controller) -> Self {
        match controller.controller_style {
            ControllerStyle::LeftJoycon => Self::JOY_LEFT,
            ControllerStyle::RightJoycon => Self::JOY_RIGHT,
            _ => Self::new(controller.buttons),
        }
    }
}
