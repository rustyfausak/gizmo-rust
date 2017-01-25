#[derive(Debug)]
pub struct Game {
    pub size1: u32,
    pub size2: u32
}

impl Game {
    pub fn new() -> Game {
        Game{
            size1: 0,
            size2: 0
        }
    }
}
