use std::{collections::HashMap, fs, io};

use crate::{food::Food, kanban::Kanbans, task::Tasks, Config, WorldResult};

pub struct World {
    pub jobs: Jobs,
    pub food: Food,
    pub tasks: Tasks,
    pub kanbans: Kanbans,
}

fn get_dir(path: &str) -> WorldResult<HashMap<String, String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| {
            (
                entry.file_name().to_str().unwrap().to_string(),
                fs::read_to_string(entry.path()).unwrap(),
            )
        })
        .collect())
}

impl World {
    pub fn new(config: Config) -> WorldResult<World> {
        let raw_jobs = get_dir(&config.directories.jobs)?;
        let raw_food = get_dir(&config.directories.food)?;
        let raw_tasks = get_dir(&config.directories.tasks)?;

        Ok(World {
            jobs: Jobs::new(raw_jobs)?,
            food: Food::new(raw_food)?,
            tasks: Tasks::new(raw_tasks)?,
            kanbans: Kanbans::new(),
        })
    }

    pub fn render(&self) -> WorldResult<()> {
        self.food.render()?;
        self.tasks.render()?;
        self.jobs.render()?;
        self.kanbans.render()?;
        Ok(())
    }
}

pub struct Jobs {}

impl Jobs {
    pub fn new(raw_jobs: HashMap<String, String>) -> WorldResult<Jobs> {
        Ok(Jobs {})
    }

    pub fn render(&self) -> WorldResult<()> {
        Ok(())
    }
}
