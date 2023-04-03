use std::fmt;
#[derive(PartialEq)]
pub enum QuestAction {
    Walk,
    Collect,
}

impl fmt::Display for QuestAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ret = match self {
            QuestAction::Walk => write!(f, "Walk"),
            QuestAction::Collect => write!(f, "Collect"),
        };
        ret
    }
}