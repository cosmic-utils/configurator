use std::fmt::Display;

use derive_more::derive::Unwrap;

use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone, Unwrap, PartialEq, Eq, Hash)]
#[unwrap(ref)]
pub enum DataPathType {
    Name(String),
    Indice(usize),
}

impl From<String> for DataPathType {
    fn from(value: String) -> Self {
        DataPathType::Name(value)
    }
}

impl From<&String> for DataPathType {
    fn from(value: &String) -> Self {
        DataPathType::Name(value.to_owned())
    }
}

impl From<&str> for DataPathType {
    fn from(value: &str) -> Self {
        DataPathType::Name(value.to_owned())
    }
}

impl From<usize> for DataPathType {
    fn from(value: usize) -> Self {
        DataPathType::Indice(value)
    }
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

pub fn data_path_alloc_one(data_path: &[DataPathType]) -> Vec<DataPathType> {
    let mut new_vec = Vec::with_capacity(data_path.len() + 1);
    new_vec.extend_from_slice(data_path);
    new_vec
}

pub fn data_path_push(
    data_path: &[DataPathType],
    more: impl Into<DataPathType>,
) -> Vec<DataPathType> {
    let mut new_vec = data_path_alloc_one(data_path);
    new_vec.push(more.into());
    new_vec
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

    pub fn last_current_data(&self) -> Option<&DataPathType> {
        self.pos.map(|pos| self.get_at(pos).unwrap())
    }

    pub fn current(&self) -> &[DataPathType] {
        match self.pos {
            Some(pos) => &self.vec[0..=pos],
            None => &[],
        }
    }

    /// Keep the maximum of path, based on nodes that still exist
    pub fn sanitize_path(&mut self, tree: &NodeContainer) {
        fn find_first_invalid_index(
            data_path: &[DataPathType],
            mut node: &NodeContainer,
        ) -> Option<usize> {
            for (pos, data) in data_path.iter().enumerate() {
                match (&node.node, data) {
                    (Node::Array(node_array), DataPathType::Indice(i))
                        if let Some(value) = &node_array.value
                            && let Some(n) = value.get(*i) =>
                    {
                        node = n;
                    }
                    (Node::Struct(node_struct), DataPathType::Name(name))
                        if let Some(field) = node_struct.fields.get(name) =>
                    {
                        node = &field.node;
                    }
                    _ => return Some(pos),
                }
            }

            None
        }

        if let Some(pos) = find_first_invalid_index(&self.vec, tree) {
            self.vec.truncate(pos);

            if pos == 0 {
                self.pos = None;
            } else {
                self.pos = self
                    .pos
                    .map(|current_pos| std::cmp::min(current_pos, pos - 1));
            }
        }
    }
}

impl NodeContainer {
    pub fn get_at<'a, 'b>(
        &'a self,
        data_path: Box<dyn Iterator<Item = &'b DataPathType> + 'b>,
    ) -> Option<&'a Self> {
        let mut node = self;

        for data in data_path {
            match (&node.node, data) {
                (Node::Array(node_array), DataPathType::Indice(pos))
                    if let Some(value) = &node_array.value
                        && let Some(n) = value.get(*pos) =>
                {
                    node = n;
                }
                (Node::Struct(node_struct), DataPathType::Name(name))
                    if let Some(field) = node_struct.fields.get(name) =>
                {
                    node = &field.node;
                }
                _ => return None,
            }
        }

        Some(node)
    }

    pub fn get_at_mut<'a, 'b>(
        &'a mut self,
        data_path: Box<dyn Iterator<Item = &'b DataPathType> + 'b>,
    ) -> Option<&'a mut Self> {
        let mut node = self;

        for data in data_path {
            match (&mut node.node, data) {
                (Node::Array(node_array), DataPathType::Indice(pos)) => {
                    if let Some(value) = &mut node_array.value
                        && let Some(n) = value.get_mut(*pos)
                    {
                        node = n;
                    } else {
                        return None;
                    }
                }
                (Node::Struct(node_struct), DataPathType::Name(name)) => {
                    if let Some(field) = node_struct.fields.get_mut(name) {
                        node = &mut field.node;
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }

        Some(node)
    }
}
