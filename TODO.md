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