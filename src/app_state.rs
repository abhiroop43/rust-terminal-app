pub struct AppState {
    pub items: Vec<String>,
    pub selected: usize,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            items: vec![
                "Exercise".to_string(),
                "Meditate".to_string(),
                "Journal".to_string(),
            ],
            selected: 0,
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }
}
