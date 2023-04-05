use std::fmt;

use lib::block::Block;

use crate::quest_action::QuestAction;
use crate::utils::QuestStatus;
use crate::quest_task::{QuestTask, ModifyTask};

pub enum Condition {
    InOrder,
    AtLeast(i32),
}
pub struct Quest {
    pub status: QuestStatus,
    pub tasks: Vec<QuestTask>,
    pub condition: Option<Condition>,
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
                task.complete_once();
            } else if &task.object == block && task.task == QuestAction::Collect {
                task.complete_once();
            }

            if task.status == QuestStatus::Complete {
                completed_tasks += 1;
            }
        }
        
        if self.tasks.len() == completed_tasks {
            match self.condition {
                Some(Condition::InOrder) => {
                    //eprintln!("{}", self.tasks.len());
                    let mut res = Vec::new();
                    res.extend(self.tasks.iter().zip(self.tasks.iter().skip(1)).filter(|(&ref a, &ref b)| a.completion_time < b.completion_time).map(|(_, b)| b));
                    eprint!("{}", res.len());
                    if res.len() == self.tasks.len() - 1 {
                        
                        self.status = QuestStatus::Complete;
                    }
                },
                Some(Condition::AtLeast(_)) => {
                    
                },
                None => self.status = QuestStatus::Complete,
            }
        }
    }

    fn reset_quest(&mut self) {
        self.status = QuestStatus::Ongoing;
        for task in self.tasks.iter_mut() {
            task.reset_task();
        }
    }

}

impl Default for Quest {

    fn default() -> Self {
        Self { status: QuestStatus::Ongoing, tasks: Vec::new() , condition: None}
    }

}

impl fmt::Display for Quest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = "".to_string();
        if self.condition.is_none() {
            for task in self.tasks.iter() {
                let add = format!("{} {}\n ^ Complete {} more time(s)\n", task.status, task, task.x_more_times);
                ret = ret + &add;
            }
        } else {
            ret = ret + &format!("{} You must, {}, complete each of these quests:\n", self.status, self.condition.as_ref().expect("Bad Error"));
            for task in self.tasks.iter() {
                let add = format!("   {} {}\n    ^ Complete {} more time(s)\n", task.status, task, task.x_more_times);
                ret = ret + &add;
            }
        }
        write!(f, "{}", ret)
    }
}      

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = match self {
            Condition::InOrder => write!(f, "in order"),
            Condition::AtLeast(_) => write!(f, "at least"),
        };
        ret
    }
}

pub fn create_quest (condition: Option<Condition>) -> Quest {
    Quest { status: QuestStatus::Ongoing, tasks: Vec::<QuestTask>::new(), condition: condition }
}