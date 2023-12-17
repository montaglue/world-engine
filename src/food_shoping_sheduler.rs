use std::collections::HashMap;

use chrono::{Datelike, Duration};
use markdown::mdast::{List, ListItem, Node, Text};

use crate::{
    food::Food,
    task::{KanbanInfo, Task},
    world::World,
};

pub fn produce_tasks_from_diches(world: &mut World) {
    let diches = &world.food;
    let mut tasks = inner_produce_tasks_from_diches(diches);
    world.tasks.tasks.extend(tasks);
}

fn get_shoping_datetime() -> chrono::NaiveDateTime {
    let now = chrono::Local::now();
    // get next thursday
    let mut date = now.date_naive();
    let mut weekday = Datelike::weekday(&date);

    while weekday != chrono::Weekday::Thu {
        date = date + Duration::days(1);
        weekday = Datelike::weekday(&date);
    }

    let datetime =
        chrono::NaiveDateTime::new(date, chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap()); // TODO: remove magical number

    datetime
}

fn inner_produce_tasks_from_diches(diches: &Food) -> Vec<(String, Task)> {
    let mut ingredients = HashMap::new();

    for day in &diches.week_dich_shedule.0 {
        for slot in &day.0 {
            for dich in &slot.1 {
                for ingredient in &diches.diches[dich.0].ingredients {
                    let amount = ingredient.amount;
                    let name = ingredient.name.clone();
                    let entry = ingredients.entry(name).or_insert(0);
                    *entry += amount;
                }
            }
        }
    }

    fn to_mdast((name, amount): (String, usize)) -> Node {
        Node::ListItem(ListItem {
            checked: Some(false),
            spread: false,
            children: vec![Node::Text(Text {
                value: format!("{}: {}", name, amount),
                position: None,
            })],
            position: None,
        })
    }

    let list = ingredients.into_iter().map(to_mdast).collect::<Vec<Node>>();

    let list = Node::List(List {
        ordered: true,
        start: None,
        spread: false,
        children: list,
        position: None,
    });

    let datetime = get_shoping_datetime();

    let name = format!("Weekly shoping {}.md", datetime.date());

    let task = Task {
        priority: 0,
        name: name.clone(),
        description: list,
        date: Some(datetime.date()),
        time: Some(datetime.time()),
        duration: Some(Duration::hours(2)), // TODO: remove magical number
        kanban: Some(KanbanInfo {
            board: "Shoping".to_string(),
            stage: "ToDo".to_string(),
        }),
    };

    vec![(name, task)]
}
