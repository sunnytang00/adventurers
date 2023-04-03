use crate::utils::QuestStatus;
use crate::quest_action::QuestAction;
use lib::block::Block;

pub struct QuestTask {
    pub status: QuestStatus,
    pub task: QuestAction,
    pub object: Block,
    pub no_of_times: i32,
    pub completed_times: i32,
}

pub trait SetTask {
    fn set_task(&mut self, task: QuestAction, no_of_times: i32);

    fn reset_task(&mut self);
}

impl SetTask for QuestTask {

    fn set_task(&mut self, task: QuestAction, no_of_times: i32) {
        self.task = task;
        self.no_of_times = no_of_times;
    }

    fn reset_task(&mut self) {
        self.status = QuestStatus::Ongoing;
        self.completed_times = 0;
    }
    
}