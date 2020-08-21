use std::{collections::HashMap, fmt::Display};

use serde::{Serialize, Deserialize};

pub trait ReceiveDamage {
    fn receive_damage(&mut self, dmg: u32) -> bool;
}

pub trait Healable {
    fn health(&self) -> i32;
    fn add_health<H>(&self, entity: &mut H) -> bool
    where
        H: ReceiveHealth,
    {
        entity.add_health(self.health())
    }
}

pub trait ReceiveHealth {
    fn add_health(&mut self, amount: i32) -> bool;
}

pub trait Damageable {
    fn damage(&self) -> u32;
    fn deal_damage<D>(&mut self, entity: &mut D) -> bool
    where
        D: ReceiveDamage,
    {
        entity.receive_damage(self.damage())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    health: u32,
    weapons: Vec<Weapon>,
    name: String,
    id: String
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Health: {})", self.name, self.health)?;
        for weapon in self.weapons.iter() {
            write!(f, "{}", weapon)?;
        }
        writeln!(f)
    }
}

impl ReceiveDamage for Entity {
    fn receive_damage(&mut self, dmg: u32) -> bool {
        self.add_health(-(dmg as i32))
    }
}

impl ReceiveHealth for Entity {
    fn add_health(&mut self, amount: i32) -> bool {
        let health: i32 = self.health as i32;
        if health + amount < 0 {
            self.health = 0;
            false
        } else {
            self.health += amount as u32;
            true
        }
    }
}

impl Entity {
    pub fn new<T: Into<String>>(name: T, health: u32) -> Self {
        Self {
            name: name.into(),
            health,
            weapons: Vec::new(),
            id: "".into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub id: String,
    pub name: String,
    pub damage: u32,
}

impl Damageable for Weapon {
    fn damage(&self) -> u32 {
        self.damage
    }
}

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}) - {}: {}", self.id, self.name, self.damage)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Potion {
    id: String,
    name: String,
    health: i32,
}

impl Healable for Potion {
    fn health(&self) -> i32 {
        self.health
    }
}
