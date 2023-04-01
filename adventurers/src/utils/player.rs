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

    //fn move(&mut self,)

    fn move_up(&mut self);

    fn move_down(&mut self);

    fn move_left(&mut self);

    fn move_right(&mut self);

    fn move_rel_up(&mut self);

    fn move_rel_down(&mut self);

    fn move_rel_left(&mut self);

    fn move_rel_right(&mut self);
}

pub trait Breath {

    fn decrease_breath(&mut self);

    fn get_breath(&self) -> i32;

    fn reset_breath(&mut self);
}

impl Movement for Player {

    fn move_up(&mut self) {
        self.y = self.y - 1;
    }

    fn move_down(&mut self) {
        self.y = self.y + 1;
    }

    fn move_left(&mut self) {
        self.x = self.x - 1;
    }

    fn move_right(&mut self) {
        self.x = self.x + 1;
    }

    fn move_rel_up(&mut self) {
        self.rel_y = self.rel_y + 1;
    }

    fn move_rel_down(&mut self) {
        self.rel_y = self.rel_y - 1;
    }

    fn move_rel_left(&mut self) {
        self.rel_x = self.rel_x - 1;
    }

    fn move_rel_right(&mut self) {
        self.rel_x = self.rel_x + 1;
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