use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The data for the player, to be serialized and deserialized across services.
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerData {
    pub skills: Vec<Skill>,
    pub inventories: HashMap<u16, Inventory>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub level: i8,
    pub xp: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub items: HashMap<u16, Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: i32,
    pub amount: i32,
}
