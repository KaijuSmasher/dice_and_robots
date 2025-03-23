use super::dice::Dice;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Roll {
    pub dice_rolled: Vec<Dice>,
    pub bonus: i32,
    pub result_rolled: i32,
}
