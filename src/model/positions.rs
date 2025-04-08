use super::roll::DiceType;

pub struct Position {
    pub dice_position: [f32; 6],
    pub roll_position: [f32; 6],
}

impl Position {
    pub fn create_position(dicetype: DiceType) -> Position {
        let dice_position = match dicetype {
            DiceType::D4 => [0.0, 0.0, -0.012229, 1.24221, 2.87813, 0.0],
            DiceType::D6 => [0.0, 0.0, -0.012229, 1.24221, 2.87813, 0.0],
            DiceType::D8 => [0.0, 0.0, -0.012229, 1.24221, 2.87813, 0.0],
            DiceType::D10 => [0.0, 0.0, -0.012229, 1.24221, 2.87813, 0.0],
            DiceType::D12 => [0.0, 0.0, -0.012229, 1.24221, 2.87813, 0.0],
            DiceType::D20 => [0.0, 0.0, -0.012229, 1.24221, 2.87813, 0.0],
        };

        Position {
            dice_position,
            roll_position: [0.0, 0.0, 0.0, 1.24221, 2.87813, 0.0],
        }
    }
}
