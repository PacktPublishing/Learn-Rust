use std::collections::BTreeMap;

fn main() {
    let mut task_scheduler: BTreeMap<i32, String> = BTreeMap::new();
    task_scheduler.insert(3, "Task A".to_string());
    task_scheduler.insert(1, "Task B".to_string());
    task_scheduler.insert(4, "Task C".to_string());
    task_scheduler.insert(2, "Task D".to_string());
    task_scheduler.insert(5, "Task E".to_string());

    extract_tasks_in_order(&mut task_scheduler);
}

fn extract_tasks_in_order(task_scheduler: &mut BTreeMap<i32, String>) {
    for (&priority, task) in task_scheduler {
        println!("Executing {} with priority {}", task, priority);
    }
}
