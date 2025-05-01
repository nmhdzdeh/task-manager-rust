use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub title: String,
    pub desc: String,
    pub completed: bool,
}
impl Task {
    pub fn create_task(title: &str, desc: &str) -> Task {
        Task {
            title: String::from(title),
            desc: String::from(desc),
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} - {}",
            if self.completed { "✅" } else { "⬜" },
            self.title,
            self.desc
        )
    }
}
