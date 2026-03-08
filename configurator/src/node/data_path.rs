use std::fmt::Display;

use derive_more::derive::Unwrap;

use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone, Unwrap, PartialEq, Eq, Hash)]
#[unwrap(ref)]
pub enum DataPathType {
    Name(String),
    Indice(usize),
}

impl Display for DataPathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataPathType::Name(name) => write!(f, "{}", name),
            DataPathType::Indice(pos) => write!(f, "{}", pos),
        }
    }
}

impl DataPathType {
    pub fn as_name(&self) -> Option<&str> {
        match self {
            DataPathType::Name(name) => Some(name.as_str()),
            DataPathType::Indice(_) => None,
        }
    }

    pub fn as_indice(&self) -> Option<usize> {
        match self {
            DataPathType::Name(_) => None,
            DataPathType::Indice(indice) => Some(*indice),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataPath {
    pub vec: Vec<DataPathType>,
    pub pos: Option<usize>,
}

impl DataPath {
    pub fn new() -> Self {
        Self {
            vec: vec![],
            pos: None,
        }
    }

    pub fn open(&mut self, field: DataPathType) {
        let next_pos = match self.pos {
            Some(pos) => pos + 1,
            None => 0,
        };

        if let Some(current) = self.get_at(next_pos)
            && current == &field
        {
            // we want the negation
        } else {
            self.vec.truncate(self.pos.map(|pos| pos + 1).unwrap_or(0));

            self.vec.push(field);
        }

        self.pos.replace(next_pos);
    }

    pub fn change_to(&mut self, pos: Option<usize>) {
        self.pos = pos;
    }

    pub fn get_at(&self, pos: usize) -> Option<&DataPathType> {
        self.vec.get(pos)
    }

    pub fn get_current(&self) -> Option<&DataPathType> {
        self.pos.map(|pos| self.get_at(pos).unwrap())
    }

    pub fn current(&self) -> &[DataPathType] {
        match self.pos {
            Some(pos) => &self.vec[0..=pos],
            None => &[],
        }
    }
}
