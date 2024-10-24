for libcosmic

- fix slider
- push multiple on Row and Column
- add_maybe for Setting Section
- on_press_with for button

- reorder array


object: validation
    il peut avoir un node_type qui specifie un "template"


enum_values: just an array of final values
can't be used with object, or array, or any validation

array: validation

instance_type
    unique type
    validate if the data is equal to any type (enum)

subschemas
    tricky. subschemas peuvent être utilisé avec n'importe quel type.
    il faut merges les subschemas dans les nodes!



algo:


deux type de node "complex"

option<(Vec<String>, Vec<String>)>

[[],["hello"]]

Enum {
    Null {},
    Array {
        type: Array {
            Array {
                type: String {}
            },
            Array {
                type: string {}
            }
        },
        max: 2,
        min: 2,
        values: [
            Array {
                type: String {},
                values: []
            },
            Array {
                type: string {},
                values: ["hello"]
            }
        ]
    }
}


Object {
    same
}




subschema: {

    all_of: {

    }
}


code

let mut node = match instance_type {

    Vec => {
        
        for a in instance {

        }
        Node::Enum
    }
    Array => {
        let node = Node::Array
        if let Some(array validation) {

            for item in items {
                add node to array
            }
        }
        node
    }



}




si on on instance type = null | string

on aura un node de type 
n = Enum {
    nodes: {
        null,
        String
    }
}

on rentre dans un one_of {

    Node::Enum(schema.map {
        node.apply(schema)
    })

}