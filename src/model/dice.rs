use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

#[derive(Serialize, Deserialize)]
pub struct Dice {
    pub dice_type: DiceType,
    pub value_rolled: i32,
}
