/// This is what a "quest" should do.
/// Note that all `Quests` implement `std::fmt::Display`
/// to show the current progress of the quest.
/// 
/// You do not have to use this trait but, this may be a useful place to start.

// pub trait Quest<Event>: std::fmt::Display {
//     /// Whenever something happens, you call "register_event" to tell the quest what's happened.
//     fn register_event(&mut self, event: &Event) -> QuestStatus;

//     /// Reset the quest, so that players can restart.
//     fn reset(&mut self);
// }

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuestStatus {
    Complete,
    Ongoing
}

impl fmt::Display for QuestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = match self {
            QuestStatus::Complete => write!(f, " ✓"),
            QuestStatus::Ongoing => write!(f, "[ ]"),
        };
        ret
    }
}

// impl From<bool> for QuestStatus {
//     fn from(status: bool) -> Self {
//         match status {
//             true => QuestStatus::Complete,
//             false => QuestStatus::Ongoing,
//         }
//     }
// }

// impl From<QuestStatus> for bool {
//     fn from(status: QuestStatus) -> Self {
//         matches!(status, QuestStatus::Complete)
//     }
// }