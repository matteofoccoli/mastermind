#[derive(PartialEq, Debug)]
pub struct Hint {
    in_right_position: u32,
    in_wrong_position: u32,
}

impl Hint {
    pub fn new(in_right_position: u32, in_wrong_position: u32) -> Hint {
        Hint {
            in_right_position,
            in_wrong_position,
        }
    }
}
