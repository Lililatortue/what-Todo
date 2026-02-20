# little project to make todo management easier
tool to look into a project tree and divide task by variable name AKA workload

## things it does
- [x] finds every todo and seperates them by var name,
- [x] opens file concerned in editor (tested very briefly in nvim)

## things that are going to be worked on atm
- [ ] adding configuration file (toml) for customizing parser
- [ ] documentation

## next up
- [ ] cleaner error handling
- [ ] TUI integration (future)

# documentation

## List command 
syntaxe: todo list <value> <flag>
description: list todos containing that value, if value is null it returns everything
flags:
  - -s -> doesnt showcase description.
  - -p <path> -> specifies a path.

## Open command 
syntaxe: todo open <value> <flag>
description: open files that have todo's in nvim, if value is null it returns every files with a todo 
flags:
  - -r -> will go search recursively through the dir
  - -p <path> -> will search a specifique relative path
