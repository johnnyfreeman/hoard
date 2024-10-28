# hoard

## Help

```
> hoard
Usage: hoard <command> <args>

Commands:
   init                 - create task storage file in the current directory
   add <description>    - adds task
   list                 - displays task list
   remove <description> - removes a task
   sort                 - sort tasks
```

## Init

Creates the file needed for task storage if one doesn't already exist.

```
hoard init
```

## Add

```
> hoard add one
> hoard add two three
```

## List

```
> hoard list
one
two
three
```

## Remove task

```
hoard remove one
```

## Sort tasks

```
hoard sort
```

## Usage with fzf

fzf makes is really easy to add/remove tasks in bulk. I would recommend creating an alias for these in your `.bashrc|.zshrc|.fshrc` file.

### Buik add tasks

```
fzf -m | hoard add
```

### Bulk remove tasks

```
task list | fzf | hoard remove
```

## Usage with bat

Pipe into bat for easier readibility.

```
hoard list | bat --language="markdown"
```

## Usage with git

If you don't want your task list to be commited, you should add the storage file to your ignore or excludes.

```
git config --global core.excludesFile '~/.gitignore'
```
