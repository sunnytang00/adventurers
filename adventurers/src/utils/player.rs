use crate::utils::direction::Direction;
pub struct Player {
    pub x: i32,
    pub y: i32,
    //Relative position are cartesian coordinates calculated from middle of termgame window
    pub rel_x: i32,
    pub rel_y: i32,
    pub breath: i32,
    pub char: char,
}

pub trait Movement {
    fn move_dir(&mut self, direction: Direction);

    fn move_dir_rel(&mut self, direction: Direction);
}

pub trait Breath {
    fn decrease_breath(&mut self);

    fn get_breath(&self) -> i32;

    fn reset_breath(&mut self);
}

impl Movement for Player {
    fn move_dir(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.x = self.x - 1,
            Direction::Right => self.x = self.x + 1,
            Direction::Up => self.y = self.y - 1,
            Direction::Down => self.y = self.y + 1,
        };
    }

    fn move_dir_rel(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.rel_x = self.rel_x - 1,
            Direction::Right => self.rel_x = self.rel_x + 1,
            Direction::Up => self.rel_y = self.rel_y + 1,
            Direction::Down => self.rel_y = self.rel_y - 1,
        };
    }
}

impl Breath for Player {
    fn decrease_breath(&mut self) {
        self.breath = self.breath - 1;
    }

    fn get_breath(&self) -> i32 {
        self.breath
    }

    fn reset_breath(&mut self) {
        self.breath = 10;
    }
}
