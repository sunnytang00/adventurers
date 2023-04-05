
use std::fmt;

use crate::{utils::QuestStatus, quest::Quest};
use crate::quest_action::QuestAction;
use lib::block::Block;
use std::time::{SystemTime};
pub struct QuestTask {
    pub status: QuestStatus,
    pub task: QuestAction,
    pub object: Block,
    pub no_of_times: i32,
    pub x_more_times: i32,
    pub completion_time: (SystemTime, bool),
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
            if self.completion_time.1 == false {
                self.completion_time = (SystemTime::now(), true);
            }
        }
    }

}

impl fmt::Display for QuestTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} a {}", self.task, self.object)
    }
}

pub fn create_task(quest: &mut Quest, action: QuestAction, block: Block, num_of_times: i32, time: SystemTime) {
    let task = QuestTask { status: QuestStatus::Ongoing, task: action, object: block, no_of_times: num_of_times, x_more_times: num_of_times, completion_time: (time, false) };
    quest.tasks.push(task);
}