use crate::utils::QuestStatus;
use crate::quest_task::QuestTask;
pub struct Quest {
    pub status: QuestStatus,
    pub tasks: Vec<QuestTask>,
}

pub trait Questing {

    fn add_tasks(&mut self, quest: QuestTask);

}

impl Questing for Quest {

    fn add_tasks(&mut self, quest: QuestTask) {
        self.tasks.push(quest);
    }
    
}

impl Default for Quest {

    fn default() -> Self {
        Self { status: QuestStatus::Ongoing, tasks: Vec::new() }
    }

}