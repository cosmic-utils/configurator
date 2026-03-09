use crate::node::{Node, NodeContainer, data_path::DataPathType};

impl NodeContainer {
    // todo: rewrite with if_let_guards
    pub fn set_modified<'a, 'b>(
        &'a mut self,
        data_path: &mut dyn Iterator<Item = &'b DataPathType>,
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
}
