# stage_divetoheart

A [Skyline](https://github.com/skyline-dev/skyline) plugin for Super Smash Bros. Ultimate that patches code and data referenced by the Hollow Bastion stage to allow for configuring the behavior of Dive to the Heart. This plugin is meant to be used alongside the [Dive to the Heart Configurator](https://gamebanana.com/mods/393112) mod.

The latest version is available from the [Releases](https://github.com/ThatNintendoNerd/stage_divetoheart/releases/latest) page.

## Configuration

When launching the game with the plugin installed, a new configuration file will be created at `sd:/ultimate/stage_divetoheart/config.toml` if it does not already exist. This file can be edited with any text editor.

Each table contains key-value pairs for each form of the stage, where the keys are as follows:

- `normal` corresponds to the normal form.
- `battle` corresponds to the Battlefield form.
- `end` corresponds to the Ω form.

The default values aim to represent as close to a vanilla experience as possible, so it is left up to the user to configure them to their liking.

### Tables

| Header | Description |
| --- | --- |
| `[override_gimmick_off]` | Determines if Dive to the Heart should appear with stage hazards disabled. |
| `[override_gimmick_on]` | Determines if Dive to the Heart should not appear with stage hazards enabled. |
| `[default_location]` | <p>The starting location of the stage.</p><p>For a list of valid values, refer to the Location column in the [Locations](#locations) section's table.</p> |
| `[default_scene]` | <p>The fallback stained-glass design.</p><p>The specified stained-glass design is considered when either an invalid or no selection command is held when selecting the stage on the Stage Select screen.</p><p>For a list of valid values, refer to the Stained Glass column in the [Stained-Glass Design Selection Commands](#stained-glass-design-selection-commands) section's table.</p> |

### Locations

| Location | Description |
| --- | --- |
| `Hollow Bastion` | Starts the stage on Hollow Bastion. |
| `Dive to the Heart` | Starts the stage on Dive to the Heart. |
| `Random` | Decides a random starting location between Hollow Bastion and Dive to the Heart. |

### Stained-Glass Design Selection Commands

To override the stained-glass design listed in the configuration file with a user's choice, hold the corresponding selection command and select Hollow Bastion on the Stage Select screen.

| Stained Glass | Selection Command |
| --- | --- |
| `Random` | None |
| `Sora` | L |
| `Riku` | L + ↑ |
| `Roxas` | L + ← |
| `Xion` | L + ↓ |
| `Terra` | L + R + ↑ |
| `Ventus` | L + R + → |
| `Aqua` | L + R + ↓ |

The listed directions refer to those on:

- The Left Stick or +Control Pad for dual Joy-Con, Nintendo Switch Pro Controller, etc.
- The Control Stick for sideways Joy-Con.
- The Control Stick for the Nintendo GameCube Controller.

## Building

NOTE: This project cannot be compiled without the libc2 library. Said library is unreleased due to its incomplete state, but its release is planned.

With an up-to-date version of the Rust toolchain installed and [cargo-skyline](https://github.com/jam1garner/cargo-skyline) 3.0.0 or newer, run the following command to compile the project in release mode:

```
cargo skyline build --release
```

The resulting build is found at `./target/aarch64-skyline-switch/release/libstage_divetoheart.nro`
