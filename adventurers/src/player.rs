pub struct Player {
    pub x: i32,
    pub y: i32,
    pub char: char,
}

pub trait Movement {
    fn moveUp(&mut self);
    fn moveDown(&mut self);
    fn moveLeft(&mut self);
    fn moveRight(&mut self);
}

impl Movement for Player {

    fn moveUp(&mut self) {
        self.y = self.y - 1;
    }

    fn moveDown(&mut self) {
        self.y = self.y + 1;
    }

    fn moveLeft(&mut self) {
        self.x = self.x - 1;
    }

    fn moveRight(&mut self) {
        self.x = self.x + 1;
    }
}