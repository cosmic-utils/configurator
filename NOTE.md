moyen de produire un node

- map
- array
- un changement de variant
- application d'un default
- 


pub fn get_node(
    schema: RustSchemaRoot,
    data_path: &[DataPathType],
) -> anyhow::Result<NodeContainer>;


## Pour String

pub fn create_value(
    data_path: &[DataPathType],
    Value,
) -> Value;


NodeString {
    pub value: Option<String>,
}


quand on change la value:
    let value = create_value(data_path, str);
    let new_value = config.merge(value);
    write(new_value)



## Pour Struct

NodeStruct {
    pub name: String,
    pub description: Option<String>,
    pub fields: IndexMap<String, StructField>,
}

StructField {
    pub description: Option<String>,
}

## Pour Map

NodeMap {
    pub fields: IndexSet<String>,
}



quand on click le +:
    let value = create_value(data_path, defaultValue);
    let new_value = config.merge(value);
    write(new_value)



## Pour Enum

NodeEnum {
    pub variant: Vec<String>,
    pub value: Option<usize>,
}



quand on click sur un variant:
    let value = create_value(data_path, variant);
    let new_value = config.merge(value);
    write(new_value)



imaginons:

#[serde(default)]
struct A {
    x: B,
}


#[serde(default)]
struct B {
    x: i32
}



pub fn get_node(
    &mut nodes: HashMap<Vec<DataPathType>, NodeContainer>
    root: &RustSchemaRoot,
    data_path: &[DataPathType],
) -> anyhow::Result<()>;

les nodes stores les "vraie" valeur
et des valeur temporaire


pub modifs: HashMap<Vec<DataPathType>, Value>,

lors d'une update:
    - on ajoute l'update a modifs
    - on créeé une c config en mergeant avec full_config 
    - on appelle get_value, on verifie que missing est vide
    - si oui, on apply cette value a tous les nodes, et on écrit c