use crate::model::roll::Roll;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub dice_to_roll: Roll,
    pub history: Vec<Roll>,
}
