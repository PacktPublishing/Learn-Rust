use std::collections::BTreeMap;

pub struct TodoList {
    tasks: BTreeMap<u32, String>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            tasks: BTreeMap::new(),
        }
    }

    pub fn add_task(&mut self, priority: u32, description: String) {
        self.tasks.insert(priority, description);
    }

    pub fn view_tasks(&self) -> Vec<(u32, String)> {
        let mut tasks_vec = Vec::new();
        for (priority, description) in &self.tasks {
            tasks_vec.push((*priority, description.clone()));
        }
        tasks_vec
    }

    pub fn remove_task(&mut self, priority: u32) {
        self.tasks.remove(&priority);
    }

    pub fn mark_completed(&mut self, priority: u32) {
        self.remove_task(priority);
    }

    pub fn view_high_priority(&self, threshold: u32) -> Vec<(u32, String)> {
        let mut tasks_vec = Vec::new();
        for (priority, description) in &self.tasks {
            if *priority <= threshold {
                tasks_vec.push((*priority, description.clone()));
            }
        }
        tasks_vec
    }

    pub fn filter_by_priority_range(&self, min_priority: u32, max_priority: u32) -> Vec<(u32, String)> {
        let mut tasks_vec = Vec::new();
        for (priority, description) in &self.tasks {
            if *priority >= min_priority && *priority <= max_priority {
                tasks_vec.push((*priority, description.clone()));
            }
        }
        tasks_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_view_tasks() {
        let mut todo_list = TodoList::new();
        todo_list.add_task(1, "Finish Rust project".to_string());
        todo_list.add_task(2, "Prepare presentation".to_string());

        let tasks = todo_list.view_tasks();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0], (1, "Finish Rust project".to_string()));
        assert_eq!(tasks[1], (2, "Prepare presentation".to_string()));
    }

    #[test]
    fn test_mark_completed() {
        let mut todo_list = TodoList::new();
        todo_list.add_task(1, "Finish Rust project".to_string());
        todo_list.add_task(2, "Prepare presentation".to_string());

        todo_list.mark_completed(1);
        let tasks = todo_list.view_tasks();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0], (2, "Prepare presentation".to_string()));
    }

    #[test]
    fn test_view_high_priority() {
        let mut todo_list = TodoList::new();
        todo_list.add_task(1, "Finish Rust project".to_string());
        todo_list.add_task(3, "Plan vacation".to_string());
        todo_list.add_task(2, "Prepare presentation".to_string());

        let high_priority_tasks = todo_list.view_high_priority(2);
        assert_eq!(high_priority_tasks.len(), 2);
        assert_eq!(high_priority_tasks[0], (1, "Finish Rust project".to_string()));
        assert_eq!(high_priority_tasks[1], (2, "Prepare presentation".to_string()));
    }

    #[test]
    fn test_filter_by_priority_range() {
        let mut todo_list = TodoList::new();
        todo_list.add_task(1, "Finish Rust project".to_string());
        todo_list.add_task(3, "Plan vacation".to_string());
        todo_list.add_task(2, "Prepare presentation".to_string());

        let filtered_tasks = todo_list.filter_by_priority_range(1, 2);
        assert_eq!(filtered_tasks.len(), 2);
        assert_eq!(filtered_tasks[0], (1, "Finish Rust project".to_string()));
        assert_eq!(filtered_tasks[1], (2, "Prepare presentation".to_string()));
    }
}
