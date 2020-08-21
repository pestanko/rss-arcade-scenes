use super::entities::{Entity, Weapon, Potion};
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub version: String,
    pub player: Entity,
    pub monsters: Vec<Entity>,
    pub weapons: Vec<Weapon>,
    pub potions: Vec<Potion>,
    pub scenes: Vec<Scene>,
}

impl Scenario {
    pub fn start(&self) -> Option<&Scene> {
        self.get_scene("start")
    }

    pub fn get_scene(&self, name: &str) -> Option<&Scene> {
        self.scenes.iter().find(|scene| scene.id == name)
    }

    pub fn get_monster(&self, name: &str) -> Option<&Entity> {
        self.monsters.iter().find(|monster| monster.id == name)
    }

    pub fn get_weapon(&self, name: &str) -> Option<&Weapon> {
        self.weapons.iter().find(|weapon| weapon.id == name)
    }

    pub fn get_potions(&self, name: &str) -> Option<&Potion> {
        self.potions.iter().find(|potion| potion.id == name)
    }


    pub fn get_scene_mut(&mut self, name: &str) -> Option<&mut Scene> {
        self.scenes.iter_mut().find(|scene| scene.id == name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    #[serde(default)]
    pub desc: String,
    #[serde(default)]
    pub options: Vec<GameOption>,
    pub next: Option<String>, // Optional next scene
    #[serde(rename = "type", default)]
    pub s_type: String,
}

impl Scene {
    pub fn get_option(&self, opt: &str) -> Option<&GameOption> {
        self.options.iter().find(|option| option.id == opt)
    }

    pub fn is_quit(&self) -> bool {
        self.s_type == "quit"
    }

    pub fn scene_type(&self) -> SceneType {
        SceneType::parse(&self.s_type)
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
        if self.is_quit() {
            writeln!(f, "Ending!")?;
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
            next: None,
            s_type: String::new(),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SceneType {
    Quit,
    Select,
    Unknown(String),
    Random(SceneTypeRandom),
    Potion(String),
    Weapon(String),
    Monster(String),
}

impl SceneType {
    fn parse(val: &str) -> SceneType {
        let val = val.to_lowercase();
        if val.is_empty() {
            return SceneType::Select;
        }
        let parts: Vec<&str> = val.split("|").collect();
        match parts.len() {
            1 => match parts[0] {
                "quit" => SceneType::Quit,
                "select" => SceneType::Select,
                _ => SceneType::Unknown(val.into()),
            },
            2 => match parts[0] {
                "random" | "rand" => {
                    SceneType::Random(SceneTypeRandom::parse(parts[1].trim().into()))
                },
                "potion"| "pot" => {
                    SceneType::Potion(parts[1].trim().into())
                },
                "weapon"| "weap" => {
                    SceneType::Weapon(parts[1].trim().into())
                }, "monster" => {
                    SceneType::Monster(parts[1].trim().into())
                },
                _ => SceneType::Unknown(val.into()),
            },
            _ => SceneType::Unknown(val.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SceneTypeRandom {
    Any,
    Monster,
    Weapon,
    Potion,
    Unknown(String),
}

impl SceneTypeRandom {
    pub fn parse(val: &str) -> SceneTypeRandom {
        match val {
            "any" => SceneTypeRandom::Any,
            "monster" => SceneTypeRandom::Monster,
            "weapon" => SceneTypeRandom::Weapon,
            "potion" => SceneTypeRandom::Potion,
            _ => SceneTypeRandom::Unknown(val.into()),
        }
    }
}
