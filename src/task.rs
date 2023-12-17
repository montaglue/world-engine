use std::{collections::HashMap, io::Write, str::FromStr};

use markdown::{mdast::Node, to_mdast, ParseOptions};

use crate::{util::remove_meta, WorldError, WorldResult};

#[derive(Debug)]
pub struct Tasks {
    pub tasks: HashMap<String, Task>,
}

impl Tasks {
    pub fn new(raw_tasks: HashMap<String, String>) -> WorldResult<Tasks> {
        Ok(Tasks {
            tasks: raw_tasks
                .into_iter()
                .map(Task::parse)
                .collect::<WorldResult<_>>()?,
        })
    }

    pub fn render(&self) -> WorldResult<()> {
        let dir = std::path::Path::new("tasks"); // TODO: use config

        for (name, task) in &self.tasks {
            let filename = dir.join(name);
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(filename)?;
            let md = task.description.to_string();
            let meta = task.create_meta();
            file.write_all(format!("{}{}", meta, md).as_bytes())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KanbanInfo {
    pub stage: String,
    pub board: String,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub priority: usize,
    pub name: String,
    pub description: Node,
    pub date: Option<chrono::NaiveDate>,
    pub time: Option<chrono::NaiveTime>,
    pub duration: Option<chrono::Duration>,
    pub kanban: Option<KanbanInfo>,
}

impl Task {
    pub fn parse((name, raw_task): (String, String)) -> WorldResult<(String, Task)> {
        let (description, meta) = remove_meta(&raw_task)?;
        let meta = meta.unwrap(); // TODO: handle error
        let kanban = if let Some(board) = meta["board"].as_str() {
            Some(KanbanInfo {
                board: board.to_owned(),
                stage: meta["stage"].as_str().unwrap().to_string(),
            })
        } else {
            None
        };
        Ok((
            name.clone(),
            Task {
                priority: meta["priority"].as_i64().unwrap() as usize,
                name,
                description: to_mdast(&description, &ParseOptions::default())
                    .map_err(WorldError::MarkdownParse)?,
                date: meta["date"]
                    .as_str()
                    .map(chrono::NaiveDate::from_str)
                    .transpose()
                    .unwrap(),
                time: meta["time"]
                    .as_str()
                    .map(chrono::NaiveTime::from_str)
                    .transpose()
                    .unwrap(),
                duration: None,
                kanban: kanban,
            },
        ))
    }

    pub fn create_meta(&self) -> String {
        let mut meta = String::new();
        meta.push_str("---\n");
        meta.push_str(&format!("priority: {}\n", self.priority));
        meta.push_str(&format!("name: {}\n", self.name));
        if let Some(date) = self.date {
            meta.push_str(&format!("date: {}\n", date));
        }
        if let Some(time) = self.time {
            meta.push_str(&format!("time: {}\n", time));
        }
        if let Some(duration) = self.duration {
            meta.push_str(&format!("duration: {}\n", duration));
        }
        meta.push_str("---\n");
        meta
    }
}
