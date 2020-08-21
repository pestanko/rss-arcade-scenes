# Simple Arcade-based Game

[![Build Status](https://travis-ci.org/pestanko/rss-arcade-scenes.svg?branch=master)](https://travis-ci.org/pestanko/rss-arcade-scenes)

This is my implementation of the first assignment for the Rust Summer School.

## Installation

You can use `cargo install`.

```bash
cargo install --git https://github.com/pestanko/rss-arcade-scenes.git --branch master
```

Or just clone the repository and run the `cargo build/run`

```bash
git clone https://github.com/pestanko/rss-arcade-scenes.git

cd rss-arcade-scenes

cargo build --release
```

## Usage

The application is taking one argument - the file with scenario, where the scenario is composition of the scenes.

### Example usage:

```bash
cargo run -- scenarios/scenario.txt
```

## Scenarios

Each scenario is one file - the program can be run with just one scenario at the same time.

Example of the scenario:

```
name: start
desc: Hello hero, would you like to kill a dragon?
option: yes | Yes | kill_dragon
option: no | No | exit
---
name: exit
desc: You are coward!
quit
---
name: kill_dragon
desc: Here is your dragon hero! What will you do?
option: die | Die | die
option: leave | Leave | exit
option: kill | Kill the dragon! | kill
---

...

```

### Format

Scenario consists from multiple scenes, scenes are separated by `---`.
Empty lines are ignored, also all the lines starting with `#` are ignored.

Each scene consists of:
- `name` - **required** - Name of the scene, it must be **unique**, if the scene name is `start` - this scene is the starting scene.
- `desc` - **required** - Description of the scene, this is the message shown to the player.
- `option` - **optional** - If the scene is not an ending scene this parameter (at least one) is required.
- `quit` - **optional** - If this parameter is present - the scene is an ending scene. It will end the game.


### Troubleshooting

In order to access the logs - you can set the `RUST_LOG` variable to desired log level, for example `debug`.

```bash
export RUST_LOG=debug
cargo run -- scenarios/scenario.txt
```
