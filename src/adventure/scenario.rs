use super::entities::{Entity, Weapon};
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub version: String,
    pub player: Entity,
    pub monsters: Vec<Entity>,
    pub weapons: Vec<Weapon>,
    pub scenes: Vec<Scene>,
}

impl Scenario {
    pub fn start(&self) -> Option<&Scene> {
        self.get_scene("start")
    }

    pub fn get_scene(&self, name: &str) -> Option<&Scene> {
        self.scenes.iter().find(|scene| scene.id == name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    #[serde(default)]
    pub desc: String,
    #[serde(default)]
    pub options: Vec<GameOption>,
    #[serde(default)]
    pub quit: bool,
    pub next: Option<String>, // Optional next scene
}

impl Scene {
    pub fn get_option(&self, opt: &str) -> Option<&GameOption> {
        self.options.iter().find(|option| option.id == opt)
    }
}

impl Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.desc)?;
        if !self.options.is_empty() {
            writeln!(f, "\nOptions:")?;
            for opt in self.options.iter() {
                writeln!(f, "{}) - {}", opt.id, opt.name)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            id: String::new(),
            desc: String::new(),
            options: Vec::new(),
            quit: false,
            next: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameOption {
    pub id: String,
    pub name: String,
    pub action: String,
}

impl GameOption {
    pub fn new(option: &str, name: &str, action: &str) -> Self {
        Self {
            id: option.into(),
            name: name.into(),
            action: action.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneAction {
    Prev,
    Next(String),
    Take(String),
    Fight(String),
    Unknown(String),
}

impl SceneAction {
    pub fn parse(s: &str) -> Self {
        let s = s.trim().to_lowercase();
        if s.is_empty() {
            return SceneAction::Unknown(s);
        }
        let parts: Vec<&str> = s.split('|').collect();
        match parts.len() {
            1 => match parts[0] {
                "prev" => SceneAction::Prev,
                "next" => SceneAction::Next(String::new()),
                _ => SceneAction::Unknown(s),
            },
            2 => match parts[0] {
                "next" => SceneAction::Next(parts[1].trim().into()),
                "take" => SceneAction::Take(parts[1].trim().into()),
                "fight" => SceneAction::Take(parts[1].trim().into()),
                _ => SceneAction::Unknown(s),
            },
            _ => SceneAction::Unknown(s),
        }
    }
}
