for libcosmic

- fix slider
- push multiple on Row and Column
- add_maybe for Setting Section
- on_press_with for button

## Test node generation

- [x] bool
- [x] string
- [ ] number
- [ ] float
- [ ] enum simple
- [ ] enum complex type
- [ ] option
- [ ] option complex type
- [ ] tuple
- [ ] vec
- [ ] hashmap
- [ ] nested struct / enum / complex

## End to End

1.  - Definir des structures
    - Generer le schema
    - generer le node
    - appliquer default
    - appliquer une valeur
    - serializer le node
    - serializer la valeur
    - assert eq

## Provider testing

simple: provider from value == a value ...
