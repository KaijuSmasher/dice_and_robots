use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

#[derive(Serialize, Deserialize)]
pub struct Roll {
    pub dice_used: DiceType,
    pub bonus: i32,
    pub result_rolled: i32,
}
