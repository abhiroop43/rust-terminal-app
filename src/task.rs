#[#[derive(debug, Clone)]]
pub struct Task {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(title: String, description: String) -> Self {
        Task {
            title,
            description,
            completed: false,           
        }
    }

    pub fn mark_complete(&mut self) {
        self.completed = true;
    }
}
