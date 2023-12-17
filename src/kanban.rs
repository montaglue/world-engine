use std::{collections::HashMap, fs, io::Write, path::Path};

use crate::WorldResult;

pub struct Kanban(HashMap<String, Vec<String>>);

pub struct Kanbans(HashMap<String, Kanban>);

impl Kanbans {
    pub fn new() -> Kanbans {
        Kanbans(HashMap::new())
    }

    pub fn add_task(&mut self, kanban_name: String, kanban_category: String, task_name: String) {
        let kanban = self.0.entry(kanban_name).or_insert(Kanban(HashMap::new()));
        let tasks = kanban.0.entry(kanban_category).or_insert(Vec::new());
        tasks.push(task_name);
    }

    pub fn get_kanban(&self, kanban_name: &str) -> Option<&Kanban> {
        self.0.get(kanban_name)
    }

    pub fn render(&self) -> WorldResult<()> {
        for dir in fs::read_dir(Path::new("."))? {
            let dir = dir?;
            if dir.file_type()?.is_file()
                && dir.file_name().to_str().unwrap().starts_with("kanban_")
            {
                fs::remove_file(dir.path())?;
            }
        }

        for (kanban_name, kanban) in &self.0 {
            let filename = format!("kanban_{}", kanban_name);
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(filename)?;
            file.write_all(format!("---\n\nkanban-plugin: basic\n\n---\n\n").as_bytes())?;
            for (category, tasks) in &kanban.0 {
                file.write_all(format!("## {}\n", category).as_bytes())?;
                for task in tasks {
                    file.write_all(format!("- [ ] [[{}]]\n", &task[..task.len() - 3]).as_bytes())?;
                }
                file.write_all(b"\n")?;
            }

            file.write_all(
                format!("\n%% kanban:settings\n```\n{{\"kanban-plugin\":\"basic\"}}\n```\n%%")
                    .as_bytes(),
            )?;
        }
        Ok(())
    }
}
