use crate::task::Task;

pub fn save_tasks_to_file(tasks: &Vec<Task>, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(&tasks).expect("Failed to serialize tasks");
    std::fs::write(path, json)
}

pub fn load_tasks_from_file(path: &str) -> Vec<Task> {
    let data = std::fs::read_to_string(path);
    match data {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| {
            println!("Error parsing file, starting with empty task list.");
            Vec::new()
        }),
        Err(_) => {
            println!("No file found, starting with empty task list.");
            Vec::new()
        }
    }
}
