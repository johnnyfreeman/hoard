# task

## Help

```
> task
Usage: task <command> <args>

Commands:
   add <description> - adds task
   list              - displays task list
   remove            - removes a task
   sort              - sort tasks
```

## Add

```
> task add one
> task add two three
```

## List

```
> task list
one
two
three
```

## Remove task

```
task remove one
```

## Sort tasks

```
task sort
```

## Usage with fzf

fzf makes is really easy to add/remove tasks in bulk. I would recommend creating an alias for these in your `.bashrc|.zshrc|.fshrc` file.

### Buik add tasks

```
fzf -m | xargs task add
```

### Bulk remove tasks

```
task list | fzf | xargs -d "\n" task remove
```

## Usage with bat

Pipe into bat for easier readibility.

```
task list | bat --language="markdown"
```

## Usage with git

If you don't want your task list to be commited, you should add the storage file to your ignore or excludes.

```
git config --global core.excludesFile '~/.gitignore'
```