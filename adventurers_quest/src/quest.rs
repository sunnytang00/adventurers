use std::fmt;

use lib::block::Block;

use crate::quest_action::QuestAction;
use crate::utils::QuestStatus;
use crate::quest_task::{QuestTask, ModifyTask};
pub struct Quest {
    pub status: QuestStatus,
    pub tasks: Vec<QuestTask>,
}

pub trait Questing {

    fn add_tasks(&mut self, quest: QuestTask);

    fn tasks_remaining(&self) -> usize;

    fn update_quests(&mut self, block: &Block);

    fn reset_quest(&mut self);

}

impl Questing for Quest {

    fn add_tasks(&mut self, quest: QuestTask) {
        self.tasks.push(quest);
    }

    fn tasks_remaining(&self) -> usize {
        self.tasks.len()
    }

    fn update_quests(&mut self, block: &Block) {
        
        let mut completed_tasks = 0;

        for task in self.tasks.iter_mut() {
            if &task.object == block && task.task == QuestAction::Walk {
                task.complete_once()
            }
            if task.status == QuestStatus::Complete {
                completed_tasks += 1;
            }
        }
        
        if self.tasks.len() == completed_tasks {
            self.status = QuestStatus::Complete;
        }
    }

    fn reset_quest(&mut self) {
        for task in self.tasks.iter_mut() {
            task.reset_task();
        }
    }

}

impl Default for Quest {

    fn default() -> Self {
        Self { status: QuestStatus::Ongoing, tasks: Vec::new() }
    }

}

impl fmt::Display for Quest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let ret = match self.tasks_remaining() {
        //     0 => {write!(f, "No tasks remaining")},
        //     1 => write!(f, "{} {}\n ^ Complete {} more time(s)", self.tasks[0].status, self.tasks[0], self.tasks[0].x_more_times),
        //     _ => write!(f, "line3\nline4"),
        // };
        // ret
        write!(f, "{} {}\n ^ Complete {} more time(s)", self.tasks[0].status, self.tasks[0], self.tasks[0].x_more_times)
    }
}      