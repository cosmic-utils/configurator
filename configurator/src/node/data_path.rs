use std::fmt::Display;

use derive_more::derive::Unwrap;

use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone, Unwrap, PartialEq, Eq)]
#[unwrap(ref)]
pub enum DataPathType {
    Name(String),
    Indice(usize),
    // todo: more complex type ? (Figment)
}

impl Display for DataPathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataPathType::Name(name) => write!(f, "{}", name),
            DataPathType::Indice(pos) => write!(f, "{}", pos),
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

impl NodeContainer {
    pub fn get_at<'a>(
        &self,
        mut data_path: impl Iterator<Item = &'a DataPathType>,
    ) -> Option<&Self> {
        match data_path.next() {
            Some(component) => match &self.node {
                Node::Object(node_object) => {
                    let name = component.unwrap_name_ref();

                    let node = node_object.nodes.get(name).unwrap();

                    node.get_at(data_path)
                }
                Node::Enum(node_enum) => {
                    let p = component.unwrap_indice_ref();
                    let node = &node_enum.nodes[*p];

                    node.get_at(data_path)
                }
                Node::Array(node_array) => {
                    let p = component.unwrap_indice_ref();
                    let node = &node_array.values.as_ref().unwrap()[*p];

                    node.get_at(data_path)
                }
                _ => panic!(),
            },
            None => Some(self),
        }
    }

    pub fn get_at_mut<'a>(
        &mut self,
        mut data_path: impl Iterator<Item = &'a DataPathType>,
    ) -> Option<&mut Self> {
        match data_path.next() {
            Some(component) => match &mut self.node {
                Node::Object(node_object) => {
                    let name = component.unwrap_name_ref();

                    let node = node_object.nodes.get_mut(name).unwrap();

                    node.get_at_mut(data_path)
                }
                Node::Enum(node_enum) => {
                    let p = component.unwrap_indice_ref();
                    let node = &mut node_enum.nodes[*p];

                    node.get_at_mut(data_path)
                }
                Node::Array(node_array) => {
                    let p = component.unwrap_indice_ref();
                    let node = &mut node_array.values.as_mut().unwrap()[*p];

                    node.get_at_mut(data_path)
                }
                _ => panic!(),
            },
            None => Some(self),
        }
    }

    pub fn set_modified<'a>(&mut self, mut data_path: impl Iterator<Item = &'a DataPathType>) {
        self.modified = true;

        if let Some(component) = data_path.next() {
            match &mut self.node {
                Node::Object(node_object) => {
                    let name = component.unwrap_name_ref();

                    let node = node_object.nodes.get_mut(name).unwrap();

                    node.set_modified(data_path);
                }
                Node::Enum(node_enum) => {
                    let p = component.unwrap_indice_ref();
                    let node = &mut node_enum.nodes[*p];

                    node.set_modified(data_path);
                }
                Node::Array(node_array) => {
                    let p = component.unwrap_indice_ref();
                    let node = &mut node_array.values.as_mut().unwrap()[*p];

                    node.set_modified(data_path);
                }
                _ => {}
            }
        }
    }
}
