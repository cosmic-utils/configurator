use crate::{
    generic_value::Value,
    node::{Node, NodeContainer, data_path::DataPathType},
};

impl NodeContainer {
    pub fn set_modified<'a>(
        &mut self,
        data_path: impl IntoIterator<Item = &'a DataPathType>,
    ) {
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
                        node = &mut field.node;
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            }
        }

        node.modified = true;
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
                        field.node.set_modified_from_value(value);
                    }
                }

                true
            }
            _ => false,
        };
    }
}
