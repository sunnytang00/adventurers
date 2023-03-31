pub struct Player {
    pub x: i32,
    pub y: i32,
    pub rel_x: i32,
    pub rel_y: i32,
    pub char: char,
}

pub trait Movement {
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_rel_up(&mut self);
    fn move_rel_down(&mut self);
    fn move_rel_left(&mut self);
    fn move_rel_right(&mut self);
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