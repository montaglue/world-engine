use std::{fs, io, str::FromStr};

use food_shoping_sheduler::produce_tasks_from_diches;

use thiserror::Error;

mod food;
mod food_shoping_sheduler;
mod kanban;
mod make_kanban_boards;
mod task;
mod util;
mod world;

pub struct Directories {
    pub jobs: String,
    pub food: String,
    pub tasks: String,
}

impl Directories {
    pub fn new() -> Directories {
        Directories {
            jobs: String::from("jobs"),
            food: String::from("food"),
            tasks: String::from("tasks"),
        }
    }
}

pub struct Config {
    directories: Directories,
}

impl FromStr for Config {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directories = Directories::new();
        Ok(Config { directories })
    }
}

impl Config {
    pub fn new(filename: &str) -> WorldResult<Config> {
        match fs::read_to_string(filename) {
            Ok(contents) => Ok(contents.parse()?),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Config {
                directories: Directories::new(),
            }),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {}

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Parse Error: {0}")]
    Parse(#[from] ParseError),
    #[error("MD parse Error: {0}")]
    MarkdownParse(String),
    #[error("Error while parsing diches")]
    ErrorWhileParsingDiches,
    #[error("Error while parsing week diches shedule")]
    ErrorWhileParsingWeekDichShedule,
    #[error("No metadata ending")]
    NoMetadataEnding,
    #[error("Yaml parsing error: {0}")]
    YamlParsingError(#[from] yaml_rust::ScanError),
}

type WorldResult<T> = Result<T, WorldError>;

fn main() -> WorldResult<()> {
    let config = Config::new("world.toml")?;

    let mut world = world::World::new(config)?;

    produce_tasks_from_diches(&mut world);
    make_kanban_boards::make_kanban_boards(&mut world);

    println!("{:?}", world.tasks.tasks);

    world.render()?;

    Ok(())
}
