use std::collections::HashMap;

use crate::{
    kanban::Kanbans,
    task::{Task, Tasks},
    world::World,
};

pub fn make_kanban_boards(world: &mut World) {
    let kanbans = inner_make_kanban_boards(&world.tasks);
    world.kanbans = kanbans;
}

fn inner_make_kanban_boards(task: &Tasks) -> Kanbans {
    let mut kanbans = Kanbans::new();
    for (name, task) in &task.tasks {
        if let Some(kanban) = &task.kanban {
            kanbans.add_task(kanban.board.clone(), kanban.stage.clone(), name.clone());
        }
    }

    kanbans
}
