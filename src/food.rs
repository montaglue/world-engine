use std::collections::HashMap;

use markdown::{mdast::Node, ParseOptions};

use crate::{
    util::{md_parse_paragraph, remove_meta, SlotShedule, Week},
    WorldError, WorldResult,
};

#[derive(Debug)]
pub struct Dich {
    pub name: String,
    pub resepy: String,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug)]
pub struct Ingredient {
    pub name: String,
    pub amount: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DichId(pub usize);

pub struct Food {
    pub week_dich_shedule: Week<SlotShedule<Vec<DichId>>>,
    pub diches: Vec<Dich>,
}

fn md_parse_diches(ast: Node) -> Option<Vec<Dich>> {
    let list = ast.children()?[0].children()?;
    let mut diches = Vec::new();

    for dich in list {
        let dich = dich.children()?;
        let name = md_parse_paragraph(&dich[0])?;

        if dich.len() < 2 {
            diches.push(Dich {
                name,
                resepy: String::new(),
                ingredients: Vec::new(),
            });
            continue;
        }

        let ingredients_md = dich[1].children()?;

        let mut ingredients = Vec::new();

        for ingredient in ingredients_md {
            let ingredient = ingredient.children()?;
            let amout = 1;
            ingredients.push(Ingredient {
                name: md_parse_paragraph(&ingredient[0])?,
                amount: amout,
            });
        }

        diches.push(Dich {
            name,
            resepy: String::new(),
            ingredients,
        });
    }

    Some(diches)
}

fn parse_diches(markdown: &str) -> WorldResult<Vec<Dich>> {
    let (markdown, _meta) = remove_meta(markdown)?;

    let ast = markdown::to_mdast(markdown, &ParseOptions::default())
        .map_err(WorldError::MarkdownParse)?;

    md_parse_diches(ast).ok_or(WorldError::ErrorWhileParsingDiches)
}

fn md_parse_week_dich_shedule(
    ast: Node,
    diches_menu: &Vec<Dich>,
) -> Option<Week<SlotShedule<Vec<DichId>>>> {
    let list = ast.children()?[0].children()?;

    let mut week = HashMap::new();

    for day in list {
        let day = day.children()?;
        let name = md_parse_paragraph(&day[0])?;
        let mut slot_shedule = SlotShedule::new();

        if day.len() < 2 {
            week.insert(name, slot_shedule);
            continue;
        }

        let slots = day[1].children()?;

        for slot in slots {
            let slot = slot.children()?;
            let name = md_parse_paragraph(&slot[0])?;

            if slot.len() < 2 {
                slot_shedule.0.push((name, Vec::new()));
                continue;
            }

            let diches = slot[1].children()?;
            let mut diches_ids = Vec::new();

            for dich in diches {
                let dich = dich.children()?;
                let dich = md_parse_paragraph(&dich[0])?;
                let dich_id = diches_menu
                    .iter()
                    .position(|d| d.name == dich)
                    .map(DichId)?;
                diches_ids.push(dich_id);
            }
            slot_shedule.0.push((name, diches_ids))
        }
        week.insert(name, slot_shedule);
    }

    Week::new(week)
}

fn parse_week_dich_shedule(
    markdown: &str,
    diches_menu: &Vec<Dich>,
) -> WorldResult<Week<SlotShedule<Vec<DichId>>>> {
    let (markdown, _meta) = remove_meta(markdown)?;

    let ast = markdown::to_mdast(markdown, &ParseOptions::default())
        .map_err(WorldError::MarkdownParse)?;

    md_parse_week_dich_shedule(ast, diches_menu).ok_or(WorldError::ErrorWhileParsingWeekDichShedule)
}

impl Food {
    pub fn new(files: HashMap<String, String>) -> WorldResult<Food> {
        let diches = parse_diches(&files["diches.md"])?;
        let week_dich_shedule = parse_week_dich_shedule(&files["week_dich_shedule.md"], &diches)?;

        Ok(Food {
            week_dich_shedule,
            diches,
        })
    }

    pub fn render(&self) -> WorldResult<()> {
        Ok(())
    }
}
