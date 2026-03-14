use crate::{
    generic_value::Value,
    node::{Node, NodeContainer, data_path::DataPathType},
};

impl NodeContainer {
    pub fn set_modified<'a>(&mut self, data_path: impl IntoIterator<Item = &'a DataPathType>) {
        let mut node = self;

        for data in data_path {
            node.modified = true;
            match (&mut node.node, data) {
                (Node::Array(node_array), DataPathType::Indice(pos)) => {
                    if let Some(value) = &mut node_array.value
                        && let Some(n) = value.get_mut(*pos)
                    {
                        node = n;
                    } else {
                        panic!()
                    }
                }
                (Node::Struct(node_struct), DataPathType::Name(name)) => {
                    if let Some(field) = node_struct.fields.get_mut(name) {
                        node = field;
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            }
        }

        node.modified = true;
    }

    pub fn set_modified2<'a>(&mut self, data_path: impl Iterator<Item = &'a DataPathType>) {
        fn inner<'a>(
            node: &mut NodeContainer,
            mut data_path: impl Iterator<Item = &'a DataPathType>,
            force: bool,
        ) {
            node.modified = true;

            match &mut node.node {
                Node::String(node_string) => {}
                Node::Array(node_array) => {
                    if let Some(value) = &mut node_array.value {
                        for node in value {
                            inner(node, std::iter::empty(), true);
                        }
                    }
                }
                Node::Struct(node_struct) => {
                    if force {
                        for field in &mut node_struct.fields {
                            inner(field.1, std::iter::empty(), true);
                        }
                    } else {
                        if let Some(data) = data_path.next() {
                            let data = data.as_name().unwrap();
                            let field = node_struct.fields.get_mut(data).unwrap();

                            inner(field, data_path, false);
                        }
                    }
                }
            }
        }

        inner(self, data_path, false);
    }

    pub fn set_unmodified(&mut self) {
        self.modified = false;

        match &mut self.node {
            Node::String(node_string) => {}
            Node::Array(node_array) => {
                if let Some(value) = &mut node_array.value {
                    for node in value {
                        node.set_unmodified();
                    }
                }
            }
            Node::Struct(node_struct) => {
                for (_, field) in &mut node_struct.fields {
                    field.set_unmodified();
                }
            }
        }
    }

    pub fn set_modified_from_value(&mut self, value: &Value) {
        self.modified = match (&mut self.node, value) {
            (Node::String(node_string), Value::String(_)) => true,
            (Node::Array(node_array), Value::Array(values)) => {
                if let Some(nodes) = &mut node_array.value {
                    nodes
                        .iter_mut()
                        .zip(values.iter())
                        .for_each(|(node, value)| {
                            node.set_modified_from_value(value);
                        });
                }

                true
            }
            (Node::Struct(node_struct), Value::Struct(_, map)) => {
                for (name, field) in &mut node_struct.fields {
                    if let Some(value) = map.0.get(name) {
                        field.set_modified_from_value(value);
                    }
                }

                true
            }
            _ => false,
        };
    }
}
