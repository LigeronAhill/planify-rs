use std::fmt::Display;

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    New,
    InProgress,
    Done,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TaskStatus::New => "Новая",
            TaskStatus::InProgress => "В работе",
            TaskStatus::Done => "Выполнена",
        };
        write!(f, "{text}")
    }
}
impl From<String> for TaskStatus {
    fn from(s: String) -> Self {
        match s.as_ref() {
            "В работе" => TaskStatus::InProgress,
            "Выполнена" => TaskStatus::Done,
            _ => TaskStatus::New,
        }
    }
}

impl Task {
    pub fn new(user_id: i64, title: String) -> Self {
        Self {
            id: 0,
            user_id,
            title,
            status: TaskStatus::New,
        }
    }
    pub fn print(&self) -> String {
        format!(
            "Задача: {title}\nСтатус: {status}",
            title = self.title,
            status = self.status
        )
    }
}
