use std::{
    collections::HashMap,
    ops::{Deref, DerefMut, Index, IndexMut},
    path::PathBuf,
};

use markdown::mdast::Node;
use yaml_rust::{Yaml, YamlLoader};

use crate::{WorldError, WorldResult};

pub struct Week<T>(pub [T; 7]);

impl<T> Week<T> {
    pub fn new(mut map: HashMap<String, T>) -> Option<Week<T>> {
        // TODO: use Result and WeekDay as indexes in map
        Some(Week([
            map.remove("Sunday")?,
            map.remove("Monday")?,
            map.remove("Tuesday")?,
            map.remove("Wednesday")?,
            map.remove("Thursday")?,
            map.remove("Friday")?,
            map.remove("Saturday")?,
        ]))
    }
}

pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl<T> Index<WeekDay> for Week<T> {
    type Output = T;

    fn index(&self, index: WeekDay) -> &Self::Output {
        match index {
            WeekDay::Sunday => &self.0[0],
            WeekDay::Monday => &self.0[1],
            WeekDay::Tuesday => &self.0[2],
            WeekDay::Wednesday => &self.0[3],
            WeekDay::Thursday => &self.0[4],
            WeekDay::Friday => &self.0[5],
            WeekDay::Saturday => &self.0[6],
        }
    }
}

impl<T> IndexMut<WeekDay> for Week<T> {
    fn index_mut(&mut self, index: WeekDay) -> &mut Self::Output {
        match index {
            WeekDay::Sunday => &mut self.0[0],
            WeekDay::Monday => &mut self.0[1],
            WeekDay::Tuesday => &mut self.0[2],
            WeekDay::Wednesday => &mut self.0[3],
            WeekDay::Thursday => &mut self.0[4],
            WeekDay::Friday => &mut self.0[5],
            WeekDay::Saturday => &mut self.0[6],
        }
    }
}

pub struct SlotShedule<T>(pub Vec<(String, T)>);

impl<T> SlotShedule<T> {
    pub fn new() -> SlotShedule<T> {
        SlotShedule(Vec::new())
    }

    pub fn get(&self, slot: &str) -> Option<&T> {
        self.0.iter().find(|(s, _)| s == slot).map(|(_, t)| t)
    }

    pub fn get_mut(&mut self, slot: &str) -> Option<&mut T> {
        self.0.iter_mut().find(|(s, _)| s == slot).map(|(_, t)| t)
    }
}

pub fn md_parse_paragraph(ast: &Node) -> Option<String> {
    match &ast.children()?[0] {
        Node::Text(s) => Some(s.value.clone()),
        _ => None,
    }
}

pub fn remove_meta(ast: &str) -> WorldResult<(&str, Option<Yaml>)> {
    if !ast.starts_with("---") {
        return Ok((ast, None));
    }

    let end_index = ast[3..].find("---").ok_or(WorldError::NoMetadataEnding)?;

    let meta = &ast[..end_index + 3];

    let ast = &ast[end_index + 6..];

    let meta = YamlLoader::load_from_str(meta)?
        .pop()
        .ok_or(WorldError::NoMetadataEnding)?;

    Ok((ast, Some(meta)))
}
