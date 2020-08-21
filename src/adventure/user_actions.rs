use shlex;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum UserAction {
    Select(String),
    Next,
    Quit,
    Inventory,
    Player,
    Scene,
    Help,
    Unknown(String),
    Empty,
}

impl UserAction {
    pub fn parse(val: &str) -> UserAction {
        let val = val.trim();
        if val.is_empty() {
            return UserAction::Empty;
        }
        let parts: Vec<String> = shlex::split(val).unwrap();
        match parts.get(0).unwrap().as_str() {
            "select" | "s" => {
                UserAction::Select(parts.get(1).map(|x| x.clone()).unwrap_or_default())
            }
            "next" | "n" => UserAction::Next,
            "player" | "p" => UserAction::Player,
            "scene" => UserAction::Scene,
            "help" | "?" => UserAction::Help,
            "quit" | "q" => UserAction::Quit,
            _ => UserAction::Unknown(val.into()),
        }
    }
}

pub fn prompt_action() -> std::io::Result<UserAction> {
    print!(">>> ");
    std::io::stdout().flush()?;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let line = buffer.trim();
    Ok(UserAction::parse(&line))
}
