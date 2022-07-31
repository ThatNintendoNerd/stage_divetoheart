# stage_divetoheart

A [Skyline](https://github.com/skyline-dev/skyline) plugin for Super Smash Bros. Ultimate that patches code and data referenced by the Hollow Bastion stage to allow for configuring the behavior of Dive to the Heart. This plugin is meant to be used alongside this [stage mod](https://gamebanana.com/mods/393112).

The latest release can be found [here](https://github.com/ThatNintendoNerd/stage_divetoheart/releases/latest).

## Configuration

When starting the game with this plugin installed, a new configuration file will be created at `sd:/ultimate/config_dth.toml` if it does not already exist. This file can be edited with any text editor.

Each table has a corresponding field for each form of the stage. The default values aim to represent as much of a vanilla experience as possible, so it is left up to the user to configure the values to their liking.

### Tables

| Header | Description |
| --- | --- |
| `[hazards_off_override]` | Determines if Dive to the Heart can appear with hazards off. |
| `[default_location]` | Determines whether the stage should start on `"Hollow Bastion"` or `"Dive to the Heart"`. The text encapsulated with an inline code block are the valid values that can be used. |
| `[default_station]` | Determines the fallback station to default to if no station override button combination was used on the Stage Select screen. For a list of valid values, refer to the table's Station column in the [Station Override Button Combinations](#station-override-button-combinations) section. |

### Station Override Button Combinations

To override the station listed in the configuration file with a user's choice, hold the corresponding controller button combination and select Hollow Bastion on the Stage Select screen.

| Station | Override Combination |
| --- | --- |
| `Random` | None |
| `Sora` | L |
| `Riku` | L & D-Pad Up |
| `Roxas` | L & D-Pad Left |
| `Xion` | L & D-Pad Down |
| `Terra` | R & D-Pad Up |
| `Ventus` | R & D-Pad Right |
| `Aqua` | R & D-Pad Down |

## Building

NOTE: This project cannot be compiled without the smash_stage library. Said library is unreleased due to its incomplete state, but its release is planned for the future.

With an up-to-date version of the Rust toolchain installed and [cargo-skyline](https://github.com/jam1garner/cargo-skyline) 3.0.0 or newer, run the following command to compile the project in release mode:

```
cargo skyline build --release
```

The resulting build is found at `./target/aarch64-skyline-switch/release/libstage_divetoheart.nro`
