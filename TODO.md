for libcosmic

- fix slider
- push multiple on Row and Column
- add_maybe for Setting Section
- on_press_with for button



[config](../../../.config/configurator/configurator.json)

[schema](../../../.local/share/configurator/io.github.wiiznokes.configurator.json)


solution 1: modified est appliqué dès le root
problems: 
- quand on set qq chose a default, on veut modifier seulement a partir du node en question
advantages:
- shortcircuit algos


solution 2: modified est appliqué a partir du node n

on veut:
- 