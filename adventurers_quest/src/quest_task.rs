use std::fmt;

use crate::utils::QuestStatus;
use crate::quest_action::QuestAction;
use lib::block::Block;

pub struct QuestTask {
    pub status: QuestStatus,
    pub task: QuestAction,
    pub object: Block,
    pub no_of_times: i32,
    pub x_more_times: i32,
}

pub trait ModifyTask {

    fn set_task(&mut self, task: QuestAction, no_of_times: i32);

    fn reset_task(&mut self);

    fn complete_once(&mut self);

    // fn update_quest(&mut self, block: Block);

}

impl ModifyTask for QuestTask {

    fn set_task(&mut self, task: QuestAction, no_of_times: i32) {
        self.task = task;
        self.no_of_times = no_of_times;
    }

    fn reset_task(&mut self) {
        self.status = QuestStatus::Ongoing;
        self.x_more_times = self.no_of_times;
    }

    fn complete_once(&mut self) {
        if self.x_more_times > 0 {
            self.x_more_times -= 1;
        }

        if self.x_more_times == 0 {
            self.status = QuestStatus::Complete;
        }
    }

    // fn update_quest(&mut self, block: Block) -> QuestTask {
    //     // match block {
    //     //     Block::Barrier => todo!(),
    //     //     Block::Water => {
    //     //         if self.task == QuestAction::Walk && self.object == Block::Water {
    //     //             QuestTask { status: QuestStatus::Ongoing, task: self.task, object: self.object, no_of_times: self.no_of_times, x_more_times: self.x_more_times - 1 };
    //     //     },
    //     //     Block::Grass => todo!(),
    //     //     Block::Sand => todo!(),
    //     //     Block::Rock => todo!(),
    //     //     Block::Cinderblock => todo!(),
    //     //     Block::Flowerbush => todo!(),
    //     //     Block::Empty => todo!(),
    //     //     Block::Sign(_) => todo!(),
    //     //     Block::Object(_) => todo!(),
    //     // };
    // }

}

impl fmt::Display for QuestTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on a {}", self.task, self.object)
    }
}