# hoard

**hoard** is a simple command-line task manager that helps you track and organize your to-do list efficiently. It provides a minimal set of commands to create, add, list, remove, and sort your tasks.

## Installation

To use **hoard**, you can download the binary or clone the repository and build it yourself. Ensure the executable is added to your system's `PATH` to make it accessible from anywhere.

## Help

To get started, run **hoard** to view the available commands:

```
> hoard
Usage: hoard <command> <args>

Commands:
   init                 - create task storage file in the current directory
   add <description>    - adds a new task
   list                 - displays the task list
   remove <description> - removes a task
   sort                 - sorts tasks
```

## Commands Overview

### `init`

The `init` command sets up the task storage file if one doesn't already exist in the current directory.

```
hoard init
```

This command creates a file named `.hoard_tasks` in the current directory, which will be used to store your task list.

### `add <description>`

The `add` command allows you to add one or more tasks to your list:

```
> hoard add one
> hoard add "two three"
```

Tasks are stored sequentially, and each task will appear on a new line in the task storage file.

### `list`

The `list` command displays all the tasks currently in your storage file:

```
> hoard list
one
two
three
```

This command will print each task on a new line in the order they were added.

### `remove <description>`

The `remove` command allows you to delete a specific task from your list:

```
hoard remove one
```

Note: The exact description is required to remove a task.

### `sort`

Use the `sort` command to sort your tasks alphabetically:

```
hoard sort
```

## Power User Tips

### Using **hoard** with `fzf`

**fzf** makes it really easy to add or remove tasks in bulk. It's highly recommended to create aliases in your `.bashrc`, `.zshrc`, or `.fishrc` file for more convenience.

#### Bulk Add Tasks

You can use `fzf` to quickly add multiple tasks:

```
fzf -m | hoard add
```

Select multiple lines with `fzf` and pipe them to **hoard** to add them as tasks.

#### Bulk Remove Tasks

You can also remove tasks in bulk using **fzf**:

```
hoard list | fzf -m | hoard remove
```

The `-m` flag enables multi-select mode, allowing you to select and remove multiple tasks at once.

### Using **hoard** with `bat`

Use **bat** to add syntax highlighting to your task list for better readability:

```
hoard list | bat --language="markdown"
```

This makes the output easier to read by applying a nice formatting style.

### Using **hoard** with Git

If you don't want your task list to be committed to your Git repository, add the storage file to your `.gitignore` or global excludes file:

```
git config --global core.excludesFile '~/.gitignore'
echo .hoard_tasks >> ~/.gitignore
```

This ensures that your local task list remains personal and doesn't get committed accidentally.

## Customization

Feel free to alias **hoard** commands in your shell profile for quicker access. Example:

```
alias ht="hoard"
alias htl="hoard list"
alias hta="hoard add"
alias htr="hoard remove"
alias hts="hoard sort"
```

## Contributing

Contributions are welcome! If you'd like to add more features or improve **hoard**, please submit a pull request or open an issue.

## License

**hoard** is licensed under the MIT License. See `LICENSE` for more information.
