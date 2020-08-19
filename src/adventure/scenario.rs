use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Scenario {
    pub scenes: HashMap<String, Scene>,
}

impl Scenario {
    pub fn start(&self) -> Option<&Scene> {
        self.get_scene("start")
    }

    pub fn get_scene(&self, name: &str) -> Option<&Scene> {
        self.scenes.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub name: String,
    pub desc: String,
    pub options: Vec<GameOption>,
    pub quit: bool,
}

impl Scene {
    pub fn get_option(&self, opt: &str) -> Option<&GameOption> {
        self.options.iter().find(|option| option.option == opt)
    }
}

impl Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.desc)?;
        if !self.options.is_empty() {
            writeln!(f, "\nOptions:")?;
            for opt in self.options.iter() {
                writeln!(f, "{}) - {}", opt.option, opt.name)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            name: String::new(),
            desc: String::new(),
            options: Vec::new(),
            quit: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameOption {
    pub option: String,
    pub name: String,
    pub go_to: String,
}

impl GameOption {
    /// Parse the Game option from provided value
    ///
    /// Examples (these tests will not be executed since this is binary):
    /// ```
    /// assert_eq!(GameOption::parse(""), None);
    /// assert_eq!(GameOption::parse("y|foo"), None);
    /// assert_eq!(GameOption::parse("y|"), None);
    /// assert_eq!(GameOption::parse("y|yes|exit"), Some(GameOption::new("y", "yes", "exit")));
    /// assert_eq!(GameOption::parse("y | yes |  exit  "), Some(GameOption::new("y", "yes", "exit")));
    /// assert_eq!(GameOption::parse("  y | yes |  exit  "), Some(GameOption::new("y", "yes", "exit")));
    /// assert_eq!(GameOption::parse("y||exit  "), Some(GameOption::new("y", "", "exit")));
    /// assert_eq!(GameOption::parse("y||  "), Some(GameOption::new("y", "", "")));
    /// ```
    pub fn parse(value: &str) -> Option<Self> {
        let parts: Vec<&str> = value.split('|').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Self {
            option: parts[0].trim().to_lowercase(),
            name: parts[1].trim().into(),
            go_to: parts[2].trim().to_lowercase(),
        })
    }

    pub fn new(option: &str, name: &str, go_to: &str) -> Self {
        Self {
            option: option.into(),
            name: name.into(),
            go_to: go_to.into(),
        }
    }
}
